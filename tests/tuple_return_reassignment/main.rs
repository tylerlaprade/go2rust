use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct parsed {
    pub major: Rc<RefCell<Option<String>>>,
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
    return (Rc::new(RefCell::new(Some({ let __s = (*v.borrow().as_ref().unwrap()).clone(); __s[..(1) as usize].to_string() }))), Rc::new(RefCell::new(Some({ let __s = (*v.borrow().as_ref().unwrap()).clone(); __s[(1) as usize..].to_string() }))), Rc::new(RefCell::new(Some(true))));
}

pub fn parse(mut v: Rc<RefCell<Option<String>>>) -> (Rc<RefCell<Option<parsed>>>, Rc<RefCell<Option<bool>>>) {

    let mut p = Rc::new(RefCell::new(Some(parsed { major: Rc::new(RefCell::new(Some(String::new()))) })));
    let mut ok = Rc::new(RefCell::new(Some(false)));
    { let (__tmp_0, __tmp_1, __tmp_2) = split_version(Rc::new(RefCell::new(Some((*v.borrow().as_ref().unwrap()).clone())))); let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *(*p.borrow().as_ref().unwrap()).major.borrow_mut() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *v.borrow_mut() = __moved_tmp_1; let __moved_tmp_2 = { let mut __guard = __tmp_2.borrow_mut(); __guard.take() }; *ok.borrow_mut() = __moved_tmp_2; };
    println!("{} {}", "rest:".to_string(), { let __v = (*v.borrow().as_ref().unwrap()).clone(); __v });
    return (p.clone(), ok.clone());
}

fn main() {
    let (mut p, mut ok) = parse(Rc::new(RefCell::new(Some("v1".to_string()))));
    println!("{} {}", (*(*p.borrow().as_ref().unwrap()).major.borrow().as_ref().unwrap()), { let __v = (*ok.borrow().as_ref().unwrap()).clone(); __v });
}