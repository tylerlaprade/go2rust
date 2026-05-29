use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

/// A generic function bound by an interface, instantiated with a POINTER type
/// argument (*Sq), mirroring go/ast's walkList[N Node] over []*Ident: the
/// type-param value is passed to an interface-typed parameter (as Walk(v, node)
/// does). The Rust type parameter must be the raw pointee (Sq), not the wrapped
/// handle (which doesn't satisfy the bound).
pub trait Shape: std::fmt::Display + Any {
    fn __go_clone_box_shape(&self) -> Box<dyn Shape>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_shape(&self, other: &dyn Shape) -> bool;
    fn area(&self) -> i32;
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self {
        self.__go_clone_box_shape()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sq {
    pub side: Rc<RefCell<Option<i32>>>,
}

impl Sq {
    pub fn __go_value_clone(&self) -> Self {
        Self { side: { let __guard = self.side.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Sq {
    fn default() -> Self {
        Self { side: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for Sq {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.side.borrow().as_ref().unwrap()))
    }
}


impl Sq {
    pub fn area(&self) -> i32 {
        return (*self.side.borrow().as_ref().unwrap()) * (*self.side.borrow().as_ref().unwrap());
    }
}

impl Shape for Sq {
    fn area(&self) -> i32 {
        self.area()
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone()) as Box<dyn Shape>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &dyn Shape) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Sq>() {
            self == __other
        } else {
            false
        }
    }
}

pub fn area_of(s: Rc<RefCell<Option<Box<dyn Shape>>>>) -> i32 {
    (*s.borrow().as_ref().unwrap()).area()
}

pub fn sum_all<T: Shape + Clone + 'static>(items: Rc<RefCell<Option<Vec<Rc<RefCell<Option<T>>>>>>>) -> i32 {
    let mut total = Rc::new(RefCell::new(Some(0)));
    { let __range_holder = items.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for it in __range_values.iter() {
        { let __rhs = area_of(Rc::new(RefCell::new(Some(Box::new((*it.borrow().as_ref().unwrap()).clone()) as Box<dyn Shape>)))); let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    return (*total.borrow().as_ref().unwrap());
}

fn main() {
    let mut items = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Sq { side: Rc::new(RefCell::new(Some(2))), ..Default::default() }))), Rc::new(RefCell::new(Some(Sq { side: Rc::new(RefCell::new(Some(3))), ..Default::default() })))])));
    println!("{}", format!("{}", sum_all::<Sq>(items.clone())));
}