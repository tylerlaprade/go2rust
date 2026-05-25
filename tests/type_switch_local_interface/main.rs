use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait namer: std::fmt::Display + Any {
    fn __go_clone_box_namer(&self) -> Box<dyn namer>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_namer(&self, other: &dyn namer) -> bool;
    fn name(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn namer> {
    fn clone(&self) -> Self {
        self.__go_clone_box_namer()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct alpha {
    pub name: Rc<RefCell<Option<String>>>,
}

impl alpha {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for alpha {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for alpha {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct beta {
    pub name: Rc<RefCell<Option<String>>>,
}

impl beta {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for beta {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for beta {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


impl alpha {
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }

    pub fn extra(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("{}{}", "alpha:".to_string(), (*self.name.clone().borrow().as_ref().unwrap())))));
    }
}

impl namer for alpha {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }
    fn __go_clone_box_namer(&self) -> Box<dyn namer> {
        Box::new(self.clone()) as Box<dyn namer>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_namer(&self, other: &dyn namer) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<alpha>() {
            self == __other
        } else {
            false
        }
    }
}

impl beta {
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }
}

impl namer for beta {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }
    fn __go_clone_box_namer(&self) -> Box<dyn namer> {
        Box::new(self.clone()) as Box<dyn namer>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_namer(&self, other: &dyn namer) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<beta>() {
            self == __other
        } else {
            false
        }
    }
}

pub fn describe(n: Rc<RefCell<Option<Box<dyn namer>>>>) -> Rc<RefCell<Option<String>>> {

    {
    let _ts_subject = n.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.__go_as_any());
    if _ts_val.and_then(|__v| __v.downcast_ref::<alpha>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<alpha>()).unwrap().clone())));
        drop(_ts_guard);
        return (*v.borrow().as_ref().unwrap()).extra();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<beta>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<beta>()).unwrap().clone())));
        drop(_ts_guard);
        return (*v.borrow().as_ref().unwrap()).name();;
    }
    }
    return (*n.borrow().as_ref().unwrap()).name();
}

pub fn pick_alpha() -> Rc<RefCell<Option<alpha>>> {

    return Rc::new(RefCell::new(Some(alpha { name: Rc::new(RefCell::new(Some("call".to_string()))), ..Default::default() })));
}

pub fn new_namer() -> Rc<RefCell<Option<Box<dyn namer>>>> {

    return Rc::new(RefCell::new(Some(Box::new((*pick_alpha().borrow().as_ref().unwrap()).clone()) as Box<dyn namer>)));
}

fn main() {
    let mut a = Rc::new(RefCell::new(Some(alpha { name: Rc::new(RefCell::new(Some("one".to_string()))), ..Default::default() })));
    let mut b = Rc::new(RefCell::new(Some(beta { name: Rc::new(RefCell::new(Some("two".to_string()))), ..Default::default() })));
    println!("{}", format!("{}", (*describe(Rc::new(RefCell::new(Some(Box::new((*a.borrow().as_ref().unwrap()).clone()) as Box<dyn namer>)))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*describe(Rc::new(RefCell::new(Some(Box::new((*b.borrow().as_ref().unwrap()).clone()) as Box<dyn namer>)))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*{ let __recv = new_namer(); let __result = (*__recv.borrow().as_ref().unwrap()).name(); __result }.borrow().as_ref().unwrap())));
}