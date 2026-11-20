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


#[derive(Debug, Clone, Default)]
pub struct Wrap {
    pub inner: Rc<RefCell<Option<Lit>>>,
}

impl Wrap {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.inner.clone();
        Self {
            inner: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for Wrap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.inner.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Lit {
    pub fn pos(&self) -> i64 {
        return (*self.value.borrow().as_ref().unwrap());
    }
}

impl Node for Lit {
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

#[derive(Clone)]
pub struct LitPtr(pub Rc<RefCell<Option<Lit>>>);

impl std::fmt::Display for LitPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for LitPtr {
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

pub fn dump(n: Rc<RefCell<Option<Box<dyn Node>>>>) {
    println!("{}", format!("{}", (*n.borrow().as_ref().unwrap()).pos()));
}

pub fn r#use(w: Rc<RefCell<Option<Wrap>>>) {
    dump(Rc::new(RefCell::new(Some(Box::new(LitPtr((*w.borrow().as_ref().unwrap()).inner.clone())) as Box<dyn Node>))));
}

fn main() {
    let mut w = Rc::new(RefCell::new(Some(Wrap { inner: Rc::new(RefCell::new(Some(Lit { value: Rc::new(RefCell::new(Some(42))), ..Default::default() }))).clone(), ..Default::default() })));
    r#use(w.clone());
}