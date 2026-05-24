use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};


fn format_any(value: &dyn Any) -> String {
    if let Some(v) = value.downcast_ref::<i32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<String>() {
        v.clone()
    } else if let Some(v) = value.downcast_ref::<&str>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<bool>() {
        v.to_string()
    } else {
        "<unknown>".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Spec {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Spec {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Spec {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Spec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


#[derive(Clone, Default)]
pub struct Holder {
    pub decl: Rc<RefCell<Option<Box<dyn Any>>>>,
}

impl Holder {
    pub fn __go_value_clone(&self) -> Self {
        Self { decl: self.decl.clone() }
    }
}

impl std::fmt::Display for Holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_any(self.decl.borrow().as_ref().unwrap().as_ref()))
    }
}


fn main() {
    let mut specs = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Spec { name: Rc::new(RefCell::new(Some("alpha".to_string()))), ..Default::default() }))), Rc::new(RefCell::new(Some(Spec { name: Rc::new(RefCell::new(Some("beta".to_string()))), ..Default::default() })))])));
    let mut holders = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Holder { ..Default::default() }))), Rc::new(RefCell::new(Some(Holder { ..Default::default() })))])));
    { let __range_holder = specs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, spec) in __range_values.iter().enumerate() {
        { let new_val = Box::new((*spec.borrow().as_ref().unwrap()).clone()) as Box<dyn Any>; *(*(*holders.borrow().as_ref().unwrap())[(i) as usize].clone().borrow().as_ref().unwrap()).decl.borrow_mut() = Some(new_val); };
    } }
    { let __range_holder = holders.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for h in __range_values.iter() {
        {
        let (mut s, mut ok) = ({
        let val = (*h.borrow().as_ref().unwrap()).decl.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Spec>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(Some(Default::default()))), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(Some(Default::default()))), Rc::new(RefCell::new(Some(false))))
        }
    });;
        if (*ok.borrow().as_ref().unwrap()) {
            println!("{}", format!("{}", (*(*s.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()));;
        }
    }
    } }
}