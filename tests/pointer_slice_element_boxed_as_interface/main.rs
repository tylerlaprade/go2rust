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
        Type::__go_clone_box_type_(self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Basic {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Basic {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for Basic {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Basic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Clone, Default)]
pub struct holder {
    pub typ: Rc<RefCell<Option<Box<dyn Type>>>>,
}

impl holder {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.typ.clone();
        Self {
            typ: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.typ.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
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
    {
        let mut __go_slice = Vec::<Rc<RefCell<Option<Basic>>>>::with_capacity(2);
        __go_slice.push(Rc::new(RefCell::new(Some(Basic { name: Rc::new(RefCell::new(Some("a".to_string()))), ..Default::default() }))));
        __go_slice.push(Rc::new(RefCell::new(Some(Basic { name: Rc::new(RefCell::new(Some("b".to_string()))), ..Default::default() }))));
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *Typ.borrow_mut() = Some(__go_slice);
    }
}


impl Basic {
    pub fn tag(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }
}

impl Type for Basic {
    fn tag(&self) -> Rc<RefCell<Option<String>>> {
        Basic::tag(self)
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

#[derive(Clone)]
pub struct BasicPtr(pub Rc<RefCell<Option<Basic>>>);

impl std::fmt::Display for BasicPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Type for BasicPtr {
    fn tag(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Basic::tag(__recv)
    }
    fn __go_clone_box_type_(&self) -> Box<dyn Type> {
        Box::new(self.clone()) as Box<dyn Type>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_type_(&self, other: &dyn Type) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BasicPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

fn main() {
    __go_init_all();
    let mut h: Rc<RefCell<Option<holder>>> = Rc::new(RefCell::new(Some(Default::default())));
    { let __iface_handle = Rc::new(RefCell::new(Some(Box::new(BasicPtr((*Typ.borrow().as_ref().unwrap())[(1) as usize].clone().clone())) as Box<dyn Type>))); let __iface_value = { let __iface_guard = __iface_handle.borrow(); (*__iface_guard).clone() }; *(*h.borrow_mut().as_mut().unwrap()).typ.borrow_mut() = __iface_value; };
    let mut t: Rc<RefCell<Option<Box<dyn Type>>>> = Rc::new(RefCell::new(Some(Box::new(BasicPtr((*Typ.borrow().as_ref().unwrap())[(0) as usize].clone().clone())) as Box<dyn Type>)));
    println!("{} {}", format!("{}", (*(*(*h.borrow().as_ref().unwrap()).typ.borrow().as_ref().unwrap()).tag().borrow().as_ref().unwrap())), format!("{}", (*(*t.borrow().as_ref().unwrap()).tag().borrow().as_ref().unwrap())));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
