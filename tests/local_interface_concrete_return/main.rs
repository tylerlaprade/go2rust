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

#[derive(Debug, Clone, PartialEq)]
pub struct counter {
    pub n: Rc<RefCell<Option<i32>>>,
}

impl counter {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            n: __go_clone_0_0,
        }
    }
}


impl Default for counter {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            n: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.n.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub trait Valuer: std::fmt::Display + Any {
    fn __go_clone_box_valuer(&self) -> Box<dyn Valuer>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_valuer(&self, other: &dyn Valuer) -> bool;
    fn value(&self) -> i32;
}

impl Clone for Box<dyn Valuer> {
    fn clone(&self) -> Self {
        Valuer::__go_clone_box_valuer(self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct number {
    pub n: Rc<RefCell<Option<i32>>>,
}

impl number {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            n: __go_clone_0_0,
        }
    }
}


impl Default for number {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            n: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.n.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl counter {
    pub fn read(&self) -> i32 {
        return (*self.n.borrow().as_ref().unwrap());
    }
}

impl Reader for counter {
    fn read(&self) -> i32 {
        counter::read(self)
    }
    fn __go_clone_box_reader(&self) -> Box<dyn Reader> {
        Box::new(self.clone()) as Box<dyn Reader>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_reader(&self, other: &dyn Reader) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<counter>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct counterPtr(pub Rc<RefCell<Option<counter>>>);

impl std::fmt::Display for counterPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Reader for counterPtr {
    fn read(&self) -> i32 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        counter::read(__recv)
    }
    fn __go_clone_box_reader(&self) -> Box<dyn Reader> {
        Box::new(self.clone()) as Box<dyn Reader>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_reader(&self, other: &dyn Reader) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<counterPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl number {
    pub fn value(&self) -> i32 {
        return (*self.n.borrow().as_ref().unwrap());
    }
}

impl Valuer for number {
    fn value(&self) -> i32 {
        number::value(self)
    }
    fn __go_clone_box_valuer(&self) -> Box<dyn Valuer> {
        Box::new(self.clone()) as Box<dyn Valuer>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_valuer(&self, other: &dyn Valuer) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<number>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct numberPtr(pub Rc<RefCell<Option<number>>>);

impl std::fmt::Display for numberPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Valuer for numberPtr {
    fn value(&self) -> i32 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        number::value(__recv)
    }
    fn __go_clone_box_valuer(&self) -> Box<dyn Valuer> {
        Box::new(self.clone()) as Box<dyn Valuer>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_valuer(&self, other: &dyn Valuer) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<numberPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn new_reader() -> Rc<RefCell<Option<Box<dyn Reader>>>> {
    Rc::new(RefCell::new(Some(Box::new(counterPtr(Rc::new(RefCell::new(Some(counter { n: Rc::new(RefCell::new(Some(7))), ..Default::default() }))).clone())) as Box<dyn Reader>)))
}

pub fn new_valuer() -> Rc<RefCell<Option<Box<dyn Valuer>>>> {
    Rc::new(RefCell::new(Some(Box::new(number { n: Rc::new(RefCell::new(Some(11))), ..Default::default() }) as Box<dyn Valuer>)))
}

fn main() {
    let mut reader = new_reader();
    let mut valuer = new_valuer();
    println!("{} {}", format!("{}", (*reader.borrow().as_ref().unwrap()).read()), format!("{}", (*valuer.borrow().as_ref().unwrap()).value()));
}