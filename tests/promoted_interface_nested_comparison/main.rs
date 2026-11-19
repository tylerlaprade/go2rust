mod leaf;
use leaf::*;

use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait I: std::fmt::Display + Any {
    fn __go_clone_box_i(&self) -> Box<dyn I>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_i(&self, other: &dyn I) -> bool;
    fn m(&self) -> i32;
}

impl Clone for Box<dyn I> {
    fn clone(&self) -> Self {
        I::__go_clone_box_i(self.as_ref())
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Base {
}

impl Base {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}

impl std::fmt::Display for Base {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Outer {
    pub base: Rc<RefCell<Option<Base>>>,
    pub l: Rc<RefCell<Option<Leaf>>>,
}

impl Outer {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.base.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.l.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            base: __go_clone_0_0,
            l: __go_clone_1_0,
        }
    }
}


impl Default for Outer {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Base::default())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(Leaf::default())));
        Self {
            base: __go_default_0_0,
            l: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Outer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.base.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.l.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl Base {
    pub fn m(&self) -> i32 {
        1
    }
}

impl I for Base {
    fn m(&self) -> i32 {
        Base::m(self)
    }
    fn __go_clone_box_i(&self) -> Box<dyn I> {
        Box::new(self.clone()) as Box<dyn I>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_i(&self, other: &dyn I) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Base>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct BasePtr(pub Rc<RefCell<Option<Base>>>);

impl std::fmt::Display for BasePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl I for BasePtr {
    fn m(&self) -> i32 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Base::m(__recv)
    }
    fn __go_clone_box_i(&self) -> Box<dyn I> {
        Box::new(self.clone()) as Box<dyn I>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_i(&self, other: &dyn I) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BasePtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Outer {
    pub fn m(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.base.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.m()
    }
}

impl I for Outer {
    fn m(&self) -> i32 {
        Outer::m(self)
    }
    fn __go_clone_box_i(&self) -> Box<dyn I> {
        Box::new(self.clone()) as Box<dyn I>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_i(&self, other: &dyn I) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Outer>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct OuterPtr(pub Rc<RefCell<Option<Outer>>>);

impl std::fmt::Display for OuterPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl I for OuterPtr {
    fn m(&self) -> i32 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Outer::m(__recv)
    }
    fn __go_clone_box_i(&self) -> Box<dyn I> {
        Box::new(self.clone()) as Box<dyn I>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_i(&self, other: &dyn I) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<OuterPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

fn main() {
    let mut a: Rc<RefCell<Option<Box<dyn I>>>> = Rc::new(RefCell::new(Some(Box::new(Outer { l: Rc::new(RefCell::new(Some(Leaf { n: Rc::new(RefCell::new(Some(1))), ..Default::default() }))), base: Rc::new(RefCell::new(Some(Base::default()))) }) as Box<dyn I>)));
    let mut b: Rc<RefCell<Option<Box<dyn I>>>> = Rc::new(RefCell::new(Some(Box::new(Outer { l: Rc::new(RefCell::new(Some(Leaf { n: Rc::new(RefCell::new(Some(2))), ..Default::default() }))), base: Rc::new(RefCell::new(Some(Base::default()))) }) as Box<dyn I>)));
    let mut c: Rc<RefCell<Option<Box<dyn I>>>> = Rc::new(RefCell::new(Some(Box::new(Outer { l: Rc::new(RefCell::new(Some(Leaf { n: Rc::new(RefCell::new(Some(1))), ..Default::default() }))), base: Rc::new(RefCell::new(Some(Base::default()))) }) as Box<dyn I>)));
    println!("{} {} {}", format!("{}", (*a.borrow().as_ref().unwrap()).m()), format!("{}", { let __left_holder = a.clone(); let __left_guard = __left_holder.borrow(); let __left_opt: Option<&dyn I> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = b.clone(); let __right_guard = __right_holder.borrow(); let __right_opt: Option<&dyn I> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_i(__right), _ => false }; __eq }), format!("{}", { let __left_holder = a.clone(); let __left_guard = __left_holder.borrow(); let __left_opt: Option<&dyn I> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = c.clone(); let __right_guard = __right_holder.borrow(); let __right_opt: Option<&dyn I> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_i(__right), _ => false }; __eq }));
}