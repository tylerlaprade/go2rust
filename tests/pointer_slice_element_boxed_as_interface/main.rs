use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Type: std::fmt::Display + Any {
    fn __go_clone_box_type_(&self) -> Box<dyn Type>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_type_(&self, other: &dyn Type) -> bool;
    fn tag(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn Type> {
    fn clone(&self) -> Self {
        self.__go_clone_box_type_()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Basic {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Basic {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Basic {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Basic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


#[derive(Clone, Default)]
pub struct holder {
    pub typ: Rc<RefCell<Option<Box<dyn Type>>>>,
}

impl holder {
    pub fn __go_value_clone(&self) -> Self {
        Self { typ: self.typ.clone() }
    }
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.typ.borrow().as_ref().unwrap()))
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

pub static Typ: GoGlobal<Vec<Rc<RefCell<Option<Basic>>>>> = GoGlobal::new();


fn __go_init_globals() {
    *Typ.borrow_mut() = Some(vec![]);
    *Typ.borrow_mut() = Some((*Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Basic { name: Rc::new(RefCell::new(Some("a".to_string()))), ..Default::default() }))), Rc::new(RefCell::new(Some(Basic { name: Rc::new(RefCell::new(Some("b".to_string()))), ..Default::default() })))]))).borrow().as_ref().unwrap()).clone());
}


impl Basic {
    pub fn tag(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }
}

impl Type for Basic {
    fn tag(&self) -> Rc<RefCell<Option<String>>> {
        self.tag()
    }
    fn __go_clone_box_type_(&self) -> Box<dyn Type> {
        Box::new(self.clone()) as Box<dyn Type>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_type_(&self, other: &dyn Type) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Basic>() {
            self == __other
        } else {
            false
        }
    }
}

fn main() {
    __go_init_all();
    let mut h: Rc<RefCell<Option<holder>>> = Rc::new(RefCell::new(Some(Default::default())));
    (*h.borrow_mut().as_mut().unwrap()).typ = Rc::new(RefCell::new(Some(Box::new((*(*Typ.borrow().as_ref().unwrap())[(1) as usize].clone().borrow().as_ref().unwrap()).clone()) as Box<dyn Type>)));
    let mut t: Rc<RefCell<Option<Box<dyn Type>>>> = Rc::new(RefCell::new(Some(Box::new((*(*Typ.borrow().as_ref().unwrap())[(0) as usize].clone().borrow().as_ref().unwrap()).clone()) as Box<dyn Type>)));
    println!("{} {}", format!("{}", (*(*(*h.borrow().as_ref().unwrap()).typ.borrow().as_ref().unwrap()).tag().borrow().as_ref().unwrap())), format!("{}", (*(*t.borrow().as_ref().unwrap()).tag().borrow().as_ref().unwrap())));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
