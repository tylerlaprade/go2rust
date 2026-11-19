use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait geometry: std::fmt::Display + Any {
    fn __go_clone_box_geometry(&self) -> Box<dyn geometry>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_geometry(&self, other: &dyn geometry) -> bool;
    fn area(&self) -> f64;
    fn perim(&self) -> f64;
}

impl Clone for Box<dyn geometry> {
    fn clone(&self) -> Self {
        geometry::__go_clone_box_geometry(self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct rect {
    pub width: Rc<RefCell<Option<f64>>>,
    pub height: Rc<RefCell<Option<f64>>>,
}

impl rect {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.width.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.height.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            width: __go_clone_0_0,
            height: __go_clone_0_1,
        }
    }
}


impl Default for rect {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0.0)));
        let __go_default_0_1 = Rc::new(RefCell::new(Some(0.0)));
        Self {
            width: __go_default_0_0,
            height: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.width.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.height.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl rect {
    pub fn area(&self) -> f64 {
        return (*self.width.borrow().as_ref().unwrap()) * (*self.height.borrow().as_ref().unwrap());
    }

    pub fn perim(&self) -> f64 {
        2.0 * (*self.width.borrow().as_ref().unwrap()) + 2.0 * (*self.height.borrow().as_ref().unwrap())
    }
}

impl geometry for rect {
    fn area(&self) -> f64 {
        rect::area(self)
    }
    fn perim(&self) -> f64 {
        rect::perim(self)
    }
    fn __go_clone_box_geometry(&self) -> Box<dyn geometry> {
        Box::new(self.clone()) as Box<dyn geometry>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_geometry(&self, other: &dyn geometry) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<rect>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct rectPtr(pub Rc<RefCell<Option<rect>>>);

impl std::fmt::Display for rectPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl geometry for rectPtr {
    fn area(&self) -> f64 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        rect::area(__recv)
    }
    fn perim(&self) -> f64 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        rect::perim(__recv)
    }
    fn __go_clone_box_geometry(&self) -> Box<dyn geometry> {
        Box::new(self.clone()) as Box<dyn geometry>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_geometry(&self, other: &dyn geometry) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<rectPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn measure(g: Rc<RefCell<Option<Box<dyn geometry>>>>) {
    println!("{}", format!("{}", format!("{}", (*g.borrow().as_ref().unwrap()))));
    println!("{}", format!("{}", (*g.borrow().as_ref().unwrap()).area()));
    println!("{}", format!("{}", (*g.borrow().as_ref().unwrap()).perim()));
}

fn main() {
    let mut r = Rc::new(RefCell::new(Some(rect { width: Rc::new(RefCell::new(Some(3.0))), height: Rc::new(RefCell::new(Some(4.0))), ..Default::default() })));
    measure(Rc::new(RefCell::new(Some(Box::new({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn geometry>))));
}