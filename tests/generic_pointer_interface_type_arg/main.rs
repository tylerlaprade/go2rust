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
    fn area(&self) -> i64;
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self {
        Shape::__go_clone_box_shape(self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sq {
    pub side: Rc<RefCell<Option<i64>>>,
}

impl Sq {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.side.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            side: __go_clone_0_0,
        }
    }
}


impl Default for Sq {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            side: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Sq {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.side.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Sq {
    pub fn area(&self) -> i64 {
        return (*self.side.borrow().as_ref().unwrap()) * (*self.side.borrow().as_ref().unwrap());
    }
}

impl Shape for Sq {
    fn area(&self) -> i64 {
        Sq::area(self)
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

#[derive(Clone)]
pub struct SqPtr(pub Rc<RefCell<Option<Sq>>>);

impl std::fmt::Display for SqPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Shape for SqPtr {
    fn area(&self) -> i64 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Sq::area(__recv)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone()) as Box<dyn Shape>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &dyn Shape) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SqPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn area_of(s: Rc<RefCell<Option<Box<dyn Shape>>>>) -> i64 {
    (*s.borrow().as_ref().unwrap()).area()
}

pub fn sum_all__ptr__sq(items: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Sq>>>>>>>) -> i64 {
    let mut total = Rc::new(RefCell::new(Some(0i64)));
    { let __range_holder = items.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for it in __range_values.iter() {
        { let __rhs = area_of(Rc::new(RefCell::new(Some(Box::new(SqPtr(it.clone())) as Box<dyn Shape>)))); let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    return (*total.borrow().as_ref().unwrap());
}

fn main() {
    let mut items = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Sq { side: Rc::new(RefCell::new(Some(2))), ..Default::default() }))), Rc::new(RefCell::new(Some(Sq { side: Rc::new(RefCell::new(Some(3))), ..Default::default() })))])));
    println!("{}", format!("{}", sum_all__ptr__sq(items.clone())));
}