use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Node: std::fmt::Display + Any {
    fn __go_clone_box_node(&self) -> Box<dyn Node>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_node(&self, other: &dyn Node) -> bool;
    fn count(&self) -> i64;
}

impl Clone for Box<dyn Node> {
    fn clone(&self) -> Self {
        Node::__go_clone_box_node(self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct word {
    pub text: Rc<RefCell<Option<String>>>,
}

impl word {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.text.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            text: __go_clone_0_0,
        }
    }
}


impl Default for word {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            text: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for word {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.text.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
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
        Visitor::__go_clone_box_visitor(self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct collector {
    pub total: Rc<RefCell<Option<i64>>>,
    pub hits: Rc<RefCell<Option<i64>>>,
}

impl collector {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.total.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.hits.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            total: __go_clone_0_0,
            hits: __go_clone_1_0,
        }
    }
}


impl Default for collector {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            total: __go_default_0_0,
            hits: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for collector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.total.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.hits.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl word {
    pub fn count(&self) -> i64 {
        (*self.text.borrow().as_ref().unwrap()).len() as i64
    }
}

impl Node for word {
    fn count(&self) -> i64 {
        word::count(self)
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

#[derive(Clone)]
pub struct wordPtr(pub Rc<RefCell<Option<word>>>);

impl std::fmt::Display for wordPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for wordPtr {
    fn count(&self) -> i64 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        word::count(__recv)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node> {
        Box::new(self.clone()) as Box<dyn Node>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &dyn Node) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<wordPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl collector {
    /// Inherent &mut-self method: assigns through the receiver.
    pub fn add(&mut self, n: Rc<RefCell<Option<i64>>>) {
        { let new_val = (*self.total.borrow().as_ref().unwrap()) + (*n.borrow().as_ref().unwrap()); *self.total.borrow_mut() = Some(new_val); };
        { let new_val = (*self.hits.borrow().as_ref().unwrap()) + 1; *self.hits.borrow_mut() = Some(new_val); };
    }

    /// Interface method that transitively requires a mutable receiver because it
    /// calls the &mut-self method add.
    pub fn visit(&mut self, n: Rc<RefCell<Option<Box<dyn Node>>>>) -> Rc<RefCell<Option<Box<dyn Visitor>>>> {
        self.add(Rc::new(RefCell::new(Some({ let __arg_value = (*n.borrow().as_ref().unwrap()).count(); __arg_value }))));
        if (*self.total.borrow().as_ref().unwrap()) > 100 {
        return Rc::new(RefCell::new(None));
    }
        Rc::new(RefCell::new(Some(Box::new(collectorPtr(Rc::new(RefCell::new(Some(self.clone()))))) as Box<dyn Visitor>)))
    }
}

impl Visitor for collector {
    fn visit(&mut self, n: Rc<RefCell<Option<Box<dyn Node>>>>) -> Rc<RefCell<Option<Box<dyn Visitor>>>> {
        collector::visit(self, n)
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

#[derive(Clone)]
pub struct collectorPtr(pub Rc<RefCell<Option<collector>>>);

impl std::fmt::Display for collectorPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Visitor for collectorPtr {
    fn visit(&mut self, n: Rc<RefCell<Option<Box<dyn Node>>>>) -> Rc<RefCell<Option<Box<dyn Visitor>>>> {
        let mut __recv_guard = self.0.borrow_mut();
        let __recv = __recv_guard.as_mut().unwrap();
        collector::visit(__recv, n)
    }
    fn __go_clone_box_visitor(&self) -> Box<dyn Visitor> {
        Box::new(self.clone()) as Box<dyn Visitor>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_visitor(&self, other: &dyn Visitor) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<collectorPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn walk(mut v: Rc<RefCell<Option<Box<dyn Visitor>>>>, nodes: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Box<dyn Node>>>>>>>>) {
    let mut v: Rc<RefCell<Option<Box<dyn Visitor>>>> = Rc::new(RefCell::new(v.borrow().as_ref().map(|__v| Visitor::__go_clone_box_visitor(__v.as_ref()))));
    { let __range_holder = nodes.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for n in __range_values.iter() {
        { let __iface_handle = { let __recv = v.clone(); let __result = (*__recv.borrow_mut().as_mut().unwrap()).visit(n.clone()).clone(); __result }; let __iface_value = { let __iface_guard = __iface_handle.borrow(); (*__iface_guard).clone() }; *v.borrow_mut() = __iface_value; };
        if { let __nil_result = (*v.borrow()).is_none(); __nil_result } {
        return;
    }
    } }
}

fn main() {
    let mut c = Rc::new(RefCell::new(Some(collector { total: Rc::new(RefCell::new(Some(0))), hits: Rc::new(RefCell::new(Some(0))) })));
    walk(Rc::new(RefCell::new(Some(Box::new(collectorPtr(c.clone())) as Box<dyn Visitor>))), Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Box::new(wordPtr(Rc::new(RefCell::new(Some(word { text: Rc::new(RefCell::new(Some("alpha".to_string()))), ..Default::default() }))).clone())) as Box<dyn Node>))), Rc::new(RefCell::new(Some(Box::new(wordPtr(Rc::new(RefCell::new(Some(word { text: Rc::new(RefCell::new(Some("beta".to_string()))), ..Default::default() }))).clone())) as Box<dyn Node>))), Rc::new(RefCell::new(Some(Box::new(wordPtr(Rc::new(RefCell::new(Some(word { text: Rc::new(RefCell::new(Some("gamma".to_string()))), ..Default::default() }))).clone())) as Box<dyn Node>)))]))));
    {
            let __go_print_arg_0 = format!("{}", (*(*c.borrow().as_ref().unwrap()).total.borrow().as_ref().unwrap()));
            eprintln!("{}", __go_print_arg_0)
        };
    {
            let __go_print_arg_0 = format!("{}", (*(*c.borrow().as_ref().unwrap()).hits.borrow().as_ref().unwrap()));
            eprintln!("{}", __go_print_arg_0)
        };
}