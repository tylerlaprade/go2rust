use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub value: Rc<RefCell<Option<i32>>>,
}

impl Node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            value: __go_clone_0_0,
        }
    }
}


impl Default for Node {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            value: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


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

pub(crate) struct GoGlobal<T> {
    value: std::cell::UnsafeCell<Option<T>>,
}
unsafe impl<T> Sync for GoGlobal<T> {}
impl<T> GoGlobal<T> {
    pub(crate) const fn new() -> Self {
        Self { value: std::cell::UnsafeCell::new(None) }
    }
    pub(crate) fn borrow(&'static self) -> &'static Option<T> {
        unsafe { &*self.value.get() }
    }
    pub(crate) fn borrow_mut(&'static self) -> &'static mut Option<T> {
        unsafe { &mut *self.value.get() }
    }
    pub(crate) fn clone(&'static self) -> std::rc::Rc<std::cell::RefCell<Option<T>>> where T: Clone {
        std::rc::Rc::new(std::cell::RefCell::new(self.borrow().clone()))
    }
}

pub static Current: GoGlobal<Rc<RefCell<Option<Node>>>> = GoGlobal::new();

pub static Other: GoGlobal<Rc<RefCell<Option<Node>>>> = GoGlobal::new();


pub fn __go_get_global_Current() -> Rc<RefCell<Option<Node>>> { (*Current.borrow().as_ref().unwrap()).clone() }
pub fn __go_clear_global_Current() { *Current.borrow_mut() = Some(Rc::new(RefCell::new(None))); }

pub fn __go_get_global_Other() -> Rc<RefCell<Option<Node>>> { (*Other.borrow().as_ref().unwrap()).clone() }
pub fn __go_clear_global_Other() { *Other.borrow_mut() = Some(Rc::new(RefCell::new(None))); }

fn __go_init_globals() {
    *Current.borrow_mut() = Some(Rc::new(RefCell::new(None)));
    *Other.borrow_mut() = Some(Rc::new(RefCell::new(None)));
    *Current.borrow_mut() = Some(Rc::new(RefCell::new(Some(Node { value: Rc::new(RefCell::new(Some(7))), ..Default::default() }))));
    *Other.borrow_mut() = Some(Rc::new(RefCell::new(Some(Node { value: Rc::new(RefCell::new(Some(9))), ..Default::default() }))));
}


impl Node {
    pub fn read(&self) -> i32 {
        return (*self.value.borrow().as_ref().unwrap());
    }
}

impl Reader for Node {
    fn read(&self) -> i32 {
        Node::read(self)
    }
    fn __go_clone_box_reader(&self) -> Box<dyn Reader> {
        Box::new(self.clone()) as Box<dyn Reader>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_reader(&self, other: &dyn Reader) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Node>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct NodePtr(pub Rc<RefCell<Option<Node>>>);

impl std::fmt::Display for NodePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Reader for NodePtr {
    fn read(&self) -> i32 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Node::read(__recv)
    }
    fn __go_clone_box_reader(&self) -> Box<dyn Reader> {
        Box::new(self.clone()) as Box<dyn Reader>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_reader(&self, other: &dyn Reader) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<NodePtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn get() -> Rc<RefCell<Option<Node>>> {
    (*Current.borrow().as_ref().unwrap()).clone()
}

pub fn view() -> Rc<RefCell<Option<Box<dyn Reader>>>> {
    Rc::new(RefCell::new(Some(Box::new(NodePtr({ let __arg_holder = Current.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })) as Box<dyn Reader>)))
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for Node {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
