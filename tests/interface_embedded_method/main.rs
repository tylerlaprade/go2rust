use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Node: std::fmt::Display + Any {
    fn __go_clone_box_node(&self) -> Box<dyn Node>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_node(&self, other: &dyn Node) -> bool;
    fn pos(&self) -> Rc<RefCell<Option<i32>>>;
    fn end(&self) -> Rc<RefCell<Option<i32>>>;
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

#[derive(Debug, Clone, PartialEq)]
pub struct Lit {
    pub value: Rc<RefCell<Option<i32>>>,
}

impl Lit {
    pub fn __go_value_clone(&self) -> Self {
        Self { value: { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Lit {
    fn default() -> Self {
        Self { value: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.borrow().as_ref().unwrap()))
    }
}


impl Lit {
    pub fn pos(&self) -> Rc<RefCell<Option<i32>>> {
        return self.value.clone();
    }

    pub fn end(&self) -> Rc<RefCell<Option<i32>>> {
        return Rc::new(RefCell::new(Some((*self.value.borrow().as_ref().unwrap()) + 1 as i32)));
    }

    pub fn expr_node(&self) {
    }
}

impl Expr for Lit {
    fn expr_node(&self) {
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

impl Node for Lit {
    fn pos(&self) -> Rc<RefCell<Option<i32>>> {
        return self.value.clone();
    }
    fn end(&self) -> Rc<RefCell<Option<i32>>> {
        return Rc::new(RefCell::new(Some((*self.value.borrow().as_ref().unwrap()) + 1 as i32)));
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

pub fn describe(e: &dyn Expr) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) {

    return (e.pos(), e.end());
}

fn main() {
    let mut lit = Rc::new(RefCell::new(Some(Lit { value: Rc::new(RefCell::new(Some(7 as i32))), ..Default::default() })));
    let (mut p, mut q) = describe(lit.borrow().as_ref().unwrap());
    println!("{} {}", format!("{}", { let __v = (*p.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*q.borrow().as_ref().unwrap()).clone(); __v }));
}