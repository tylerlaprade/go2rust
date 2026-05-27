use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Scope {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Scope {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Scope {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().borrow().as_ref().unwrap()))
    }
}


impl Scope {
    pub fn string(&self) -> Rc<RefCell<Option<String>>> {
        let mut buf: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some(Default::default())));
        (*buf.clone().borrow_mut().as_mut().unwrap()).push_str(&format!("scope {:p} {{", self));
        (*buf.clone().borrow_mut().as_mut().unwrap()).push_str(&format!("name={}}}", (*self.name.borrow().as_ref().unwrap())));
        return Rc::new(RefCell::new(Some({ let __builder = buf.clone(); let __guard = __builder.borrow(); let __value = (*__guard.as_ref().unwrap()).clone(); drop(__guard); __value })));
    }
}

fn main() {
    let mut s = Rc::new(RefCell::new(Some(Scope { name: Rc::new(RefCell::new(Some("outer".to_string()))), ..Default::default() })));
    let mut out = (*s.borrow().as_ref().unwrap()).string();

        // Strip the pointer (varies by run); just check the structural prefix/suffix.
    if !(*Rc::new(RefCell::new(Some({ let __s = (*out.borrow().as_ref().unwrap()).clone(); let __arg = "scope ".to_string(); __s.starts_with(&__arg) }))).borrow().as_ref().unwrap()) || !(*Rc::new(RefCell::new(Some({ let __s = (*out.borrow().as_ref().unwrap()).clone(); let __arg = " {name=outer}".to_string(); __s.contains(&__arg) }))).borrow().as_ref().unwrap()) {
        println!("{} {}", format!("{}", "FAIL:".to_string()), format!("{}", { let __v = (*out.borrow().as_ref().unwrap()).clone(); __v }));
        return;
    }
    println!("{}", format!("{}", "OK".to_string()));
}