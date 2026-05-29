use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Node: std::fmt::Display + Any {
    fn __go_clone_box_node(&self) -> Box<dyn Node>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_node(&self, other: &dyn Node) -> bool;
    fn count(&self) -> i32;
}

impl Clone for Box<dyn Node> {
    fn clone(&self) -> Self {
        self.__go_clone_box_node()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct word {
    pub text: Rc<RefCell<Option<String>>>,
}

impl word {
    pub fn __go_value_clone(&self) -> Self {
        Self { text: { let __guard = self.text.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for word {
    fn default() -> Self {
        Self { text: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for word {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.text.borrow().as_ref().unwrap()))
    }
}


pub trait Visitor: std::fmt::Display + Any {
    fn __go_clone_box_visitor(&self) -> Box<dyn Visitor>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_visitor(&self, other: &dyn Visitor) -> bool;
    fn visit(&mut self, n: Rc<RefCell<Option<Box<dyn Node>>>>) -> Rc<RefCell<Option<Box<dyn Visitor>>>>;
}

impl Clone for Box<dyn Visitor> {
    fn clone(&self) -> Self {
        self.__go_clone_box_visitor()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct collector {
    pub total: Rc<RefCell<Option<i32>>>,
    pub hits: Rc<RefCell<Option<i32>>>,
}

impl collector {
    pub fn __go_value_clone(&self) -> Self {
        Self { total: { let __guard = self.total.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, hits: { let __guard = self.hits.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for collector {
    fn default() -> Self {
        Self { total: Rc::new(RefCell::new(Some(0))), hits: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for collector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.total.borrow().as_ref().unwrap()), (*self.hits.borrow().as_ref().unwrap()))
    }
}


impl word {
    pub fn count(&self) -> i32 {
        (*self.text.borrow().as_ref().unwrap()).len() as i32
    }
}

impl Node for word {
    fn count(&self) -> i32 {
        self.count()
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node> {
        Box::new(self.clone()) as Box<dyn Node>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &dyn Node) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<word>() {
            self == __other
        } else {
            false
        }
    }
}

impl collector {
    /// Inherent &mut-self method: assigns through the receiver.
    pub fn add(&mut self, n: Rc<RefCell<Option<i32>>>) {
        { let new_val = (*self.total.borrow().as_ref().unwrap()) + (*n.borrow().as_ref().unwrap()); *self.total.borrow_mut() = Some(new_val); };
        { let new_val = (*self.hits.borrow().as_ref().unwrap()) + 1 as i32; *self.hits.borrow_mut() = Some(new_val); };
    }

    /// Interface method that transitively requires a mutable receiver because it
    /// calls the &mut-self method add.
    pub fn visit(&mut self, n: Rc<RefCell<Option<Box<dyn Node>>>>) -> Rc<RefCell<Option<Box<dyn Visitor>>>> {
        self.add(Rc::new(RefCell::new(Some((*n.borrow().as_ref().unwrap()).count()))));
        if (*self.total.borrow().as_ref().unwrap()) > 100 as i32 {
        return Rc::new(RefCell::new(None));
    }
        Rc::new(RefCell::new(Some(Box::new(self.clone()) as Box<dyn Visitor>)))
    }
}

impl Visitor for collector {
    fn visit(&mut self, n: Rc<RefCell<Option<Box<dyn Node>>>>) -> Rc<RefCell<Option<Box<dyn Visitor>>>> {
        self.visit(n)
    }
    fn __go_clone_box_visitor(&self) -> Box<dyn Visitor> {
        Box::new(self.clone()) as Box<dyn Visitor>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_visitor(&self, other: &dyn Visitor) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<collector>() {
            self == __other
        } else {
            false
        }
    }
}

pub fn walk(mut v: Rc<RefCell<Option<Box<dyn Visitor>>>>, nodes: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Box<dyn Node>>>>>>>>) {
    let mut v: Rc<RefCell<Option<Box<dyn Visitor>>>> = v.clone();
    { let __range_holder = nodes.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for n in __range_values.iter() {
        v = { let __recv = v.clone(); let __result = (*__recv.borrow_mut().as_mut().unwrap()).visit(n.clone()).clone(); __result };
        if (*v.borrow()).is_none() {
        return;
    }
    } }
}

fn main() {
    let mut c = Rc::new(RefCell::new(Some(collector { total: Rc::new(RefCell::new(Some(0))), hits: Rc::new(RefCell::new(Some(0))) })));
    walk(Rc::new(RefCell::new(Some(Box::new((*c.borrow().as_ref().unwrap()).clone()) as Box<dyn Visitor>))), Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Box::new(word { text: Rc::new(RefCell::new(Some("alpha".to_string()))), ..Default::default() }) as Box<dyn Node>))), Rc::new(RefCell::new(Some(Box::new(word { text: Rc::new(RefCell::new(Some("beta".to_string()))), ..Default::default() }) as Box<dyn Node>))), Rc::new(RefCell::new(Some(Box::new(word { text: Rc::new(RefCell::new(Some("gamma".to_string()))), ..Default::default() }) as Box<dyn Node>)))]))));
    eprintln!("{}", format!("{}", (*(*c.borrow().as_ref().unwrap()).total.borrow().as_ref().unwrap())));
    eprintln!("{}", format!("{}", (*(*c.borrow().as_ref().unwrap()).hits.borrow().as_ref().unwrap())));
}