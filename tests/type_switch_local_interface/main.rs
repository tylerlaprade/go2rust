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
        namer::__go_clone_box_namer(self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct alpha {
    pub name: Rc<RefCell<Option<String>>>,
}

impl alpha {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for alpha {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for alpha {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct beta {
    pub name: Rc<RefCell<Option<String>>>,
}

impl beta {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for beta {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for beta {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl alpha {
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some({ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }

    pub fn extra(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("{}{}", "alpha:".to_string(), (*self.name.clone().borrow().as_ref().unwrap())))))
    }
}

impl namer for alpha {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        alpha::name(self)
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

#[derive(Clone)]
pub struct alphaPtr(pub Rc<RefCell<Option<alpha>>>);

impl std::fmt::Display for alphaPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl namer for alphaPtr {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        alpha::name(__recv)
    }
    fn __go_clone_box_namer(&self) -> Box<dyn namer> {
        Box::new(self.clone()) as Box<dyn namer>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_namer(&self, other: &dyn namer) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<alphaPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl beta {
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some({ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }
}

impl namer for beta {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        beta::name(self)
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

#[derive(Clone)]
pub struct betaPtr(pub Rc<RefCell<Option<beta>>>);

impl std::fmt::Display for betaPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl namer for betaPtr {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        beta::name(__recv)
    }
    fn __go_clone_box_namer(&self) -> Box<dyn namer> {
        Box::new(self.clone()) as Box<dyn namer>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_namer(&self, other: &dyn namer) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<betaPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
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
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn namer>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<alpha>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<alpha>()).unwrap().clone())));
        return (*v.borrow().as_ref().unwrap()).extra();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<beta>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<beta>()).unwrap().clone())));
        return (*v.borrow().as_ref().unwrap()).name();;
    }
    }
    (*n.borrow().as_ref().unwrap()).name()
}

pub fn pick_alpha() -> Rc<RefCell<Option<alpha>>> {
    Rc::new(RefCell::new(Some(alpha { name: Rc::new(RefCell::new(Some("call".to_string()))), ..Default::default() })))
}

pub fn new_namer() -> Rc<RefCell<Option<Box<dyn namer>>>> {
    Rc::new(RefCell::new(Some(Box::new((*pick_alpha().borrow().as_ref().unwrap()).clone()) as Box<dyn namer>)))
}

fn main() {
    let mut a = Rc::new(RefCell::new(Some(alpha { name: Rc::new(RefCell::new(Some("one".to_string()))), ..Default::default() })));
    let mut b = Rc::new(RefCell::new(Some(beta { name: Rc::new(RefCell::new(Some("two".to_string()))), ..Default::default() })));
    println!("{}", format!("{}", (*describe(Rc::new(RefCell::new(Some(Box::new({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn namer>)))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*describe(Rc::new(RefCell::new(Some(Box::new({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn namer>)))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*{ let __recv = new_namer(); let __result = (*__recv.borrow().as_ref().unwrap()).name(); __result }.borrow().as_ref().unwrap())));
}