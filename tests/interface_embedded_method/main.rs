use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Node: std::fmt::Display + Any {
    fn __go_clone_box_node(&self) -> Box<dyn Node>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_node(&self, other: &dyn Node) -> bool;
    fn pos(&self) -> i64;
    fn end(&self) -> i64;
}

impl Clone for Box<dyn Node> {
    fn clone(&self) -> Self {
        Node::__go_clone_box_node(self.as_ref())
    }
}

pub trait Expr: Node + std::fmt::Display + Any {
    fn __go_clone_box_expr(&self) -> Box<dyn Expr>;
    fn __go_eq_expr(&self, other: &dyn Expr) -> bool;
    fn expr_node(&self);
}

impl Clone for Box<dyn Expr> {
    fn clone(&self) -> Self {
        Expr::__go_clone_box_expr(self.as_ref())
    }
}

impl Node for Box<dyn Expr> {
    fn __go_clone_box_node(&self) -> Box<dyn Node> {
        Box::new((*self).clone()) as Box<dyn Node>
    }
    fn __go_as_any(&self) -> &dyn Any {
        (**self).__go_as_any()
    }
    fn __go_eq_node(&self, other: &dyn Node) -> bool {
        (**self).__go_eq_node(other)
    }
    fn end(&self) -> i64 {
        (**self).end()
    }
    fn pos(&self) -> i64 {
        (**self).pos()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lit {
    pub value: Rc<RefCell<Option<i64>>>,
}

impl Lit {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            value: __go_clone_0_0,
        }
    }
}


impl Default for Lit {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            value: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Lit {
    pub fn pos(&self) -> i64 {
        return (*self.value.borrow().as_ref().unwrap());
    }

    pub fn end(&self) -> i64 {
        return (*self.value.borrow().as_ref().unwrap()) + 1;
    }

    pub fn expr_node(&self) {
    }
}

impl Expr for Lit {
    fn expr_node(&self) {
        Lit::expr_node(self)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) as Box<dyn Expr>
    }
    fn __go_eq_expr(&self, other: &dyn Expr) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Lit>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct LitPtr(pub Rc<RefCell<Option<Lit>>>);

impl std::fmt::Display for LitPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Expr for LitPtr {
    fn expr_node(&self) {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Lit::expr_node(__recv)
    }
    fn __go_clone_box_expr(&self) -> Box<dyn Expr> {
        Box::new(self.clone()) as Box<dyn Expr>
    }
    fn __go_eq_expr(&self, other: &dyn Expr) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<LitPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for Lit {
    fn end(&self) -> i64 {
        Lit::end(self)
    }
    fn pos(&self) -> i64 {
        Lit::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node> {
        Box::new(self.clone()) as Box<dyn Node>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &dyn Node) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Lit>() {
            self == __other
        } else {
            false
        }
    }
}

impl Node for LitPtr {
    fn end(&self) -> i64 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Lit::end(__recv)
    }
    fn pos(&self) -> i64 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Lit::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node> {
        Box::new(self.clone()) as Box<dyn Node>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &dyn Node) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<LitPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn describe(e: Rc<RefCell<Option<Box<dyn Expr>>>>) -> (i64, i64) {
    ((*e.borrow().as_ref().unwrap()).pos(), (*e.borrow().as_ref().unwrap()).end())
}

fn main() {
    let mut lit = Rc::new(RefCell::new(Some(Lit { value: Rc::new(RefCell::new(Some(7))), ..Default::default() })));
    let (mut p, mut q) = describe(Rc::new(RefCell::new(Some(Box::new(LitPtr(lit.clone())) as Box<dyn Expr>))));
    println!("{} {}", format!("{}", p), format!("{}", q));
}