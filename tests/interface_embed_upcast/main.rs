use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Node: std::fmt::Display + Any {
    fn __go_clone_box_node(&self) -> Box<dyn Node>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_node(&self, other: &dyn Node) -> bool;
    fn pos(&self) -> i32;
}

impl Clone for Box<dyn Node> {
    fn clone(&self) -> Self {
        self.__go_clone_box_node()
    }
}

pub trait Expr: Node + std::fmt::Display + Any {
    fn __go_clone_box_expr(&self) -> Box<dyn Expr>;
    fn __go_eq_expr(&self, other: &dyn Expr) -> bool;
    fn expr_node(&self);
}

impl Clone for Box<dyn Expr> {
    fn clone(&self) -> Self {
        self.__go_clone_box_expr()
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
    fn pos(&self) -> i32 {
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
        self.__go_clone_box_stmt()
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
    fn pos(&self) -> i32 {
        (**self).pos()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ident {
    pub pos: Rc<RefCell<Option<i32>>>,
}

impl Ident {
    pub fn __go_value_clone(&self) -> Self {
        Self { pos: { let __guard = self.pos.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Ident {
    fn default() -> Self {
        Self { pos: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.pos.borrow().as_ref().unwrap()))
    }
}


#[derive(Clone, Default)]
pub struct ExprStmt {
    pub x: Rc<RefCell<Option<Box<dyn Expr>>>>,
}

impl ExprStmt {
    pub fn __go_value_clone(&self) -> Self {
        Self { x: self.x.clone() }
    }
}

impl std::fmt::Display for ExprStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.x.borrow().as_ref().unwrap()))
    }
}


impl Ident {
    pub fn pos(&self) -> i32 {
        (*self.pos.borrow().as_ref().unwrap())
    }

    pub fn expr_node(&self) {
    }
}

impl Expr for Ident {
    fn expr_node(&self) {
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

impl Node for Ident {
    fn pos(&self) -> i32 {
        (*self.pos.borrow().as_ref().unwrap())
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

impl ExprStmt {
    pub fn pos(&self) -> i32 {
        (*self.x.borrow().as_ref().unwrap()).pos()
    }

    pub fn stmt_node(&self) {
    }
}

impl Node for ExprStmt {
    fn pos(&self) -> i32 {
        (*self.x.borrow().as_ref().unwrap()).pos()
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

impl Stmt for ExprStmt {
    fn stmt_node(&self) {
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
    let mut id = Rc::new(RefCell::new(Some(Ident { pos: Rc::new(RefCell::new(Some(42 as i32))), ..Default::default() })));
    walk_expr(Rc::new(RefCell::new(Some(Box::new((*id.borrow().as_ref().unwrap()).clone()) as Box<dyn Expr>))));

    let mut st = Rc::new(RefCell::new(Some(ExprStmt { x: Rc::new(RefCell::new(Some(Box::new((*id.borrow().as_ref().unwrap()).clone()) as Box<dyn Expr>))), ..Default::default() })));
    walk_stmt(Rc::new(RefCell::new(Some(Box::new((*st.borrow().as_ref().unwrap()).clone()) as Box<dyn Stmt>))));
}