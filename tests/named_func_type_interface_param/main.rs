use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display};
use std::rc::{Rc};

pub trait Node: std::fmt::Display + Any {
    fn __go_clone_box_node(&self) -> Box<dyn Node>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_node(&self, other: &dyn Node) -> bool;
    fn name(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn Node> {
    fn clone(&self) -> Self {
        self.__go_clone_box_node()
    }
}

/// inspector is a named function type whose parameter is an interface,
/// mirroring go/ast's `type inspector func(Node) bool`. Assigning a plain
/// func(Node) bool value to inspector requires the named-type definition and
/// function value to use the same interface-parameter representation.
pub type inspector = Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<Box<dyn Node>>>>) -> bool>>>>;


pub trait inspectorMethods {
    fn visit(&self, n: Rc<RefCell<Option<Box<dyn Node>>>>) -> bool;
}

impl inspectorMethods for inspector {
    fn visit(&self, n: Rc<RefCell<Option<Box<dyn Node>>>>) -> bool {
        { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<Box<dyn Node>>>>) -> bool> = { let mut __f_guard = self.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<Box<dyn Node>>>>) -> bool> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(n.clone()) }
    }
}

fn main() {
    let mut insp: inspector = Rc::new(RefCell::new(Some(Box::new(move |n: Rc<RefCell<Option<Box<dyn Node>>>>| -> bool {
        println!("{}", format!("{}", (*(*n.borrow().as_ref().unwrap()).name().borrow().as_ref().unwrap())));
        true
    }) as Box<dyn FnMut(Rc<RefCell<Option<Box<dyn Node>>>>) -> bool>)));
    if (*insp.borrow()).is_some() {
        println!("{}", format!("{}", "assigned".to_string()));
    }
}