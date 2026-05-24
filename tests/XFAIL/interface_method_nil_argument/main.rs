use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display};
use std::rc::{Rc};

pub trait Node: std::fmt::Display + Any {
    fn __go_clone_box_node(&self) -> Box<dyn Node>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_node(&self, other: &dyn Node) -> bool;
    fn node_name(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn Node> {
    fn clone(&self) -> Self {
        self.__go_clone_box_node()
    }
}

pub fn describe(node: &dyn Node) {
    if false {
        println!("{}", format!("{}", "nil node".to_string()));
        return;
    }
    println!("{}", format!("{}", (*node.node_name().borrow().as_ref().unwrap())));
}

fn main() {
    describe(nil.borrow().as_ref().unwrap());
}