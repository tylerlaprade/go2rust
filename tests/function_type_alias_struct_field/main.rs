use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub type ReportFunc = Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> ()>>>>;


#[derive(Clone)]
pub struct importer {
    pub reportf: ReportFunc,
    pub name: Rc<RefCell<Option<String>>>,
}

impl importer {
    pub fn __go_value_clone(&self) -> Self {
        Self { reportf: self.reportf.clone(), name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for importer {
    fn default() -> Self {
        Self { reportf: Rc::new(RefCell::new(None)), name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for importer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", "<func>", (*self.name.borrow().as_ref().unwrap()))
    }
}


pub fn report(label: Rc<RefCell<Option<String>>>, values: Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) {
    println!("{} {}", format!("{}", { let __v = (*label.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*values.borrow().as_ref().unwrap()).len()));
}

fn main() {
    let mut p = Rc::new(RefCell::new(Some(importer { reportf: Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<String>>>, __arg1: Rc<RefCell<Option<Vec<Box<dyn Any>>>>>| { report(__arg0, __arg1) }) as Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> ()>))), name: Rc::new(RefCell::new(Some("alpha".to_string()))), ..Default::default() })));
    { let __f_holder = (*p.borrow().as_ref().unwrap()).reportf.clone(); let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)((*p.borrow().as_ref().unwrap()).name.clone(), Rc::new(RefCell::new(Some(vec![Box::new(1) as Box<dyn Any>, Box::new("x".to_string()) as Box<dyn Any>])))) };
}