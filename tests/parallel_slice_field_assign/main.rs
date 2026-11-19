use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct parsed {
    pub kind: Rc<RefCell<Option<String>>>,
    pub rest: Rc<RefCell<Option<String>>>,
}

impl parsed {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.kind.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.rest.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            kind: __go_clone_0_0,
            rest: __go_clone_1_0,
        }
    }
}


impl Default for parsed {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            kind: __go_default_0_0,
            rest: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for parsed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.kind.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.rest.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub fn split(mut x: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<parsed>>> {
    let mut p: Rc<RefCell<Option<parsed>>> = Rc::new(RefCell::new(Some(Default::default())));
    {
        let __tmp_0 = Rc::new(RefCell::new(Some({ let __s = &((*x.borrow().as_ref().unwrap()).clone()); let __high = (1) as usize; __s[..__high].to_string() })));
        let __tmp_1 = Rc::new(RefCell::new(Some({ let __s = &((*x.borrow().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() })));
        *(*p.borrow().as_ref().unwrap()).kind.borrow_mut() = __tmp_0.borrow_mut().take();
        *x.borrow_mut() = __tmp_1.borrow_mut().take();
    };
    { let new_val = x.borrow().as_ref().unwrap().clone(); *(*p.borrow().as_ref().unwrap()).rest.borrow_mut() = Some(new_val); };
    return { let __owned = p.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
}

fn main() {
    let mut p = split(Rc::new(RefCell::new(Some("abc".to_string()))));
    println!("{}", format!("{}", (*(*p.borrow().as_ref().unwrap()).kind.borrow().as_ref().unwrap()).clone()));
    println!("{}", format!("{}", (*(*p.borrow().as_ref().unwrap()).rest.borrow().as_ref().unwrap()).clone()));
}