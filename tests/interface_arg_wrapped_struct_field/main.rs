use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Node: std::fmt::Display + Any {
    fn __go_clone_box_node(&self) -> Box<dyn Node>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_node(&self, other: &dyn Node) -> bool;
    fn pos(&self) -> Rc<RefCell<Option<i32>>>;
}

impl Clone for Box<dyn Node> {
    fn clone(&self) -> Self {
        self.__go_clone_box_node()
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


#[derive(Debug, Clone, Default)]
pub struct Wrap {
    pub inner: Rc<RefCell<Option<Lit>>>,
}

impl Wrap {
    pub fn __go_value_clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

impl std::fmt::Display for Wrap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.inner.borrow().as_ref().unwrap()))
    }
}


impl Lit {
    pub fn pos(&self) -> Rc<RefCell<Option<i32>>> {
        return self.value.clone();
    }
}

impl Node for Lit {
    fn pos(&self) -> Rc<RefCell<Option<i32>>> {
        return self.value.clone();
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

pub fn dump(n: Rc<RefCell<Option<Box<dyn Node>>>>) {
    println!("{}", format!("{}", (*(*n.borrow().as_ref().unwrap()).pos().borrow().as_ref().unwrap())));
}

pub fn r#use(w: Rc<RefCell<Option<Wrap>>>) {
    dump(Rc::new(RefCell::new(Some(Box::new((*(*w.borrow().as_ref().unwrap()).inner.borrow().as_ref().unwrap()).clone()) as Box<dyn Node>))));
}

fn main() {
    let mut w = Rc::new(RefCell::new(Some(Wrap { inner: Rc::new(RefCell::new(Some(Lit { value: Rc::new(RefCell::new(Some(42 as i32))), ..Default::default() }))).clone(), ..Default::default() })));
    r#use(w.clone());
}