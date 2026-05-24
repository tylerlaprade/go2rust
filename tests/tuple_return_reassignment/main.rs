use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct parsed {
    pub major: Rc<RefCell<Option<String>>>,
}

impl parsed {
    pub fn __go_value_clone(&self) -> Self {
        Self { major: { let __guard = self.major.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for parsed {
    fn default() -> Self {
        Self { major: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for parsed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.major.borrow().as_ref().unwrap()))
    }
}


pub fn split_version(v: Rc<RefCell<Option<String>>>) -> (Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>, Rc<RefCell<Option<bool>>>) {

    if ((*v.borrow().as_ref().unwrap()).len() as i32) == (0 as i32) {
        return (Rc::new(RefCell::new(Some("".to_string()))), Rc::new(RefCell::new(Some("".to_string()))), Rc::new(RefCell::new(Some(false))));
    }
    return (Rc::new(RefCell::new(Some({ let __s = &((*v.borrow().as_ref().unwrap()).clone()); __s[..(1) as usize].to_string() }))), Rc::new(RefCell::new(Some({ let __s = &((*v.borrow().as_ref().unwrap()).clone()); __s[(1) as usize..].to_string() }))), Rc::new(RefCell::new(Some(true))));
}

pub fn parse(mut v: Rc<RefCell<Option<String>>>) -> (Rc<RefCell<Option<parsed>>>, Rc<RefCell<Option<bool>>>) {

    let mut p = Rc::new(RefCell::new(Some(parsed { major: Rc::new(RefCell::new(Some(String::new()))) })));
    let mut ok = Rc::new(RefCell::new(Some(false)));
    { let (__tmp_0, __tmp_1, __tmp_2) = split_version(Rc::new(RefCell::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *(*p.borrow().as_ref().unwrap()).major.borrow_mut() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *v.borrow_mut() = __moved_tmp_1; let __moved_tmp_2 = { let mut __guard = __tmp_2.borrow_mut(); __guard.take() }; *ok.borrow_mut() = __moved_tmp_2; };
    println!("{} {}", format!("{}", "rest:".to_string()), format!("{}", { let __v = (*v.borrow().as_ref().unwrap()).clone(); __v }));
    return (Rc::new(RefCell::new(Some(p.borrow().as_ref().unwrap().clone()))), Rc::new(RefCell::new(Some(ok.borrow().as_ref().unwrap().clone()))));
}

fn main() {
    let (mut p, mut ok) = parse(Rc::new(RefCell::new(Some("v1".to_string()))));
    println!("{} {}", format!("{}", (*(*p.borrow().as_ref().unwrap()).major.borrow().as_ref().unwrap()).clone()), format!("{}", { let __v = (*ok.borrow().as_ref().unwrap()).clone(); __v }));
}