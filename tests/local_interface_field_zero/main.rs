use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Reader: std::fmt::Display + Any {
    fn __go_clone_box_reader(&self) -> Box<dyn Reader>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_reader(&self, other: &dyn Reader) -> bool;
    fn read(&self) -> i32;
}

impl Clone for Box<dyn Reader> {
    fn clone(&self) -> Self {
        Reader::__go_clone_box_reader(self.as_ref())
    }
}

#[derive(Clone)]
pub struct holder {
    pub reader: Rc<RefCell<Option<Box<dyn Reader>>>>,
    pub count: Rc<RefCell<Option<i32>>>,
}

impl holder {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.reader.clone();
        let __go_clone_1_0 = { let __guard = self.count.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            reader: __go_clone_0_0,
            count: __go_clone_1_0,
        }
    }
}


impl Default for holder {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(None));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            reader: __go_default_0_0,
            count: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.reader.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.count.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub fn zero_holder() -> Rc<RefCell<Option<holder>>> {
    Rc::new(RefCell::new(Some(holder { reader: Default::default(), count: Rc::new(RefCell::new(Some(0))) })))
}

fn main() {
    let mut holder = zero_holder();
    println!("{}", format!("{}", (*(*holder.borrow().as_ref().unwrap()).count.borrow().as_ref().unwrap())));
}