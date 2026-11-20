use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Node: std::fmt::Display + Any {
    fn __go_clone_box_node(&self) -> Box<dyn Node>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_node(&self, other: &dyn Node) -> bool;
    fn pos(&self) -> i64;
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
    fn pos(&self) -> i64 {
        (**self).pos()
    }
}

pub trait Stmt: Node + std::fmt::Display + Any {
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt>;
    fn __go_eq_stmt(&self, other: &dyn Stmt) -> bool;
    fn stmt_node(&self);
}

impl Clone for Box<dyn Stmt> {
    fn clone(&self) -> Self {
        Stmt::__go_clone_box_stmt(self.as_ref())
    }
}

impl Node for Box<dyn Stmt> {
    fn __go_clone_box_node(&self) -> Box<dyn Node> {
        Box::new((*self).clone()) as Box<dyn Node>
    }
    fn __go_as_any(&self) -> &dyn Any {
        (**self).__go_as_any()
    }
    fn __go_eq_node(&self, other: &dyn Node) -> bool {
        (**self).__go_eq_node(other)
    }
    fn pos(&self) -> i64 {
        (**self).pos()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ident {
    pub pos: Rc<RefCell<Option<i64>>>,
}

impl Ident {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.pos.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            pos: __go_clone_0_0,
        }
    }
}


impl Default for Ident {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            pos: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.pos.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Clone, Default)]
pub struct ExprStmt {
    pub x: Rc<RefCell<Option<Box<dyn Expr>>>>,
}

impl ExprStmt {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.x.clone();
        Self {
            x: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for ExprStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Ident {
    pub fn pos(&self) -> i64 {
        return (*self.pos.borrow().as_ref().unwrap());
    }

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
    fn __go_eq_expr(&self, other: &dyn Expr) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<IdentPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Node for Ident {
    fn pos(&self) -> i64 {
        Ident::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node> {
        Box::new(self.clone()) as Box<dyn Node>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &dyn Node) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Ident>() {
            self == __other
        } else {
            false
        }
    }
}

impl Node for IdentPtr {
    fn pos(&self) -> i64 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Ident::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node> {
        Box::new(self.clone()) as Box<dyn Node>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &dyn Node) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<IdentPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl ExprStmt {
    pub fn pos(&self) -> i64 {
        (*self.x.borrow().as_ref().unwrap()).pos()
    }

    pub fn stmt_node(&self) {
    }
}

impl Node for ExprStmt {
    fn pos(&self) -> i64 {
        ExprStmt::pos(self)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node> {
        Box::new(self.clone()) as Box<dyn Node>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &dyn Node) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ExprStmt>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct ExprStmtPtr(pub Rc<RefCell<Option<ExprStmt>>>);

impl std::fmt::Display for ExprStmtPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for ExprStmtPtr {
    fn pos(&self) -> i64 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        ExprStmt::pos(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node> {
        Box::new(self.clone()) as Box<dyn Node>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &dyn Node) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ExprStmtPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Stmt for ExprStmt {
    fn stmt_node(&self) {
        ExprStmt::stmt_node(self)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt> {
        Box::new(self.clone()) as Box<dyn Stmt>
    }
    fn __go_eq_stmt(&self, other: &dyn Stmt) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ExprStmt>() {
            false
        } else {
            false
        }
    }
}

impl Stmt for ExprStmtPtr {
    fn stmt_node(&self) {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        ExprStmt::stmt_node(__recv)
    }
    fn __go_clone_box_stmt(&self) -> Box<dyn Stmt> {
        Box::new(self.clone()) as Box<dyn Stmt>
    }
    fn __go_eq_stmt(&self, other: &dyn Stmt) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ExprStmtPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn dump_node(n: Rc<RefCell<Option<Box<dyn Node>>>>) {
    println!("{} {}", format!("{}", "node pos:".to_string()), format!("{}", (*n.borrow().as_ref().unwrap()).pos()));
}

pub fn walk_expr(e: Rc<RefCell<Option<Box<dyn Expr>>>>) {
    dump_node({ let __inner: Box<dyn Node> = (*e.borrow().as_ref().unwrap()).clone(); Rc::new(RefCell::new(Some(__inner))) });
}

pub fn walk_stmt(s: Rc<RefCell<Option<Box<dyn Stmt>>>>) {
    dump_node({ let __inner: Box<dyn Node> = (*s.borrow().as_ref().unwrap()).clone(); Rc::new(RefCell::new(Some(__inner))) });
}

fn main() {
    let mut id = Rc::new(RefCell::new(Some(Ident { pos: Rc::new(RefCell::new(Some(42))), ..Default::default() })));
    walk_expr(Rc::new(RefCell::new(Some(Box::new(IdentPtr(id.clone())) as Box<dyn Expr>))));

    let mut st = Rc::new(RefCell::new(Some(ExprStmt { x: Rc::new(RefCell::new(Some(Box::new(IdentPtr(id.clone())) as Box<dyn Expr>))), ..Default::default() })));
    walk_stmt(Rc::new(RefCell::new(Some(Box::new(ExprStmtPtr(st.clone())) as Box<dyn Stmt>))));
}