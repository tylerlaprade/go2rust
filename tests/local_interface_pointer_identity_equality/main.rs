use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Object: std::fmt::Display + Any {
    fn __go_clone_box_object(&self) -> Box<dyn Object>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_object(&self, other: &dyn Object) -> bool;
    fn name(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn Object> {
    fn clone(&self) -> Self {
        self.__go_clone_box_object()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct item {
    pub name: Rc<RefCell<Option<String>>>,
}

impl item {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for item {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


impl item {
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }
}

impl Object for item {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        item::name(self)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object> {
        Box::new(self.clone()) as Box<dyn Object>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &dyn Object) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<item>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct itemPtr(pub Rc<RefCell<Option<item>>>);

impl std::fmt::Display for itemPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{}", __v), None => write!(f, "<nil>") }
    }
}

impl Object for itemPtr {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        item::name(__recv)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object> {
        Box::new(self.clone()) as Box<dyn Object>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &dyn Object) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<itemPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn same(a: Rc<RefCell<Option<Box<dyn Object>>>>, b: Rc<RefCell<Option<Box<dyn Object>>>>) -> bool {
    { let __left_holder = a.clone(); let __left_guard = __left_holder.borrow(); let __left_opt: Option<&dyn Object> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = b.clone(); let __right_guard = __right_holder.borrow(); let __right_opt: Option<&dyn Object> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_object(__right), _ => false }; __eq }
}

fn main() {
    let mut first = Rc::new(RefCell::new(Some(item { name: Rc::new(RefCell::new(Some("a".to_string()))), ..Default::default() })));
    let mut alias = first.clone();
    let mut other = Rc::new(RefCell::new(Some(item { name: Rc::new(RefCell::new(Some("a".to_string()))), ..Default::default() })));

    let mut a: Rc<RefCell<Option<Box<dyn Object>>>> = Rc::new(RefCell::new(Some(Box::new(itemPtr(first.clone())) as Box<dyn Object>)));
    let mut b: Rc<RefCell<Option<Box<dyn Object>>>> = Rc::new(RefCell::new(Some(Box::new(itemPtr(alias.clone())) as Box<dyn Object>)));
    let mut c: Rc<RefCell<Option<Box<dyn Object>>>> = Rc::new(RefCell::new(Some(Box::new(itemPtr(other.clone())) as Box<dyn Object>)));

    println!("{} {} {}", format!("{}", (*(*a.borrow().as_ref().unwrap()).name().borrow().as_ref().unwrap())), format!("{}", (*(*b.borrow().as_ref().unwrap()).name().borrow().as_ref().unwrap())), format!("{}", (*(*c.borrow().as_ref().unwrap()).name().borrow().as_ref().unwrap())));
    println!("{} {} {}", format!("{}", same(a.clone(), b.clone())), format!("{}", same(a.clone(), c.clone())), format!("{}", (*a.borrow()).is_none()));
}