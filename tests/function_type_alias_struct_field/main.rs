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
        let __go_clone_0_0 = self.reportf.clone();
        let __go_clone_1_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            reportf: __go_clone_0_0,
            name: __go_clone_1_0,
        }
    }
}


impl Default for importer {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(None));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            reportf: __go_default_0_0,
            name: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for importer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", "<func>");
        let __go_fmt_1 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub fn report(label: Rc<RefCell<Option<String>>>, values: Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) {
    println!("{} {}", format!("{}", { let __v = (*label.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*values.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
}

fn main() {
    let mut p = Rc::new(RefCell::new(Some(importer { reportf: Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<String>>>, __arg1: Rc<RefCell<Option<Vec<Box<dyn Any>>>>>| { report(__arg0, __arg1) }) as Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> ()>))), name: Rc::new(RefCell::new(Some("alpha".to_string()))), ..Default::default() })));
    { let __f_holder = (*p.borrow().as_ref().unwrap()).reportf.clone(); let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some({ let __selector_holder = (*p.borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Rc::new(RefCell::new(Some(vec![Box::new(1i64) as Box<dyn Any>, Box::new("x".to_string()) as Box<dyn Any>])))) };
}