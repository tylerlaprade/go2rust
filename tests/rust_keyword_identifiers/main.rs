use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Branch {
    pub r#else: Rc<RefCell<Option<i32>>>,
}

impl Branch {
    pub fn __go_value_clone(&self) -> Self {
        Self { r#else: { let __guard = self.r#else.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Branch {
    fn default() -> Self {
        Self { r#else: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.r#else.borrow().as_ref().unwrap()))
    }
}


pub fn use_keyword_names(r#fn: Rc<RefCell<Option<i32>>>) -> i32 {

    let mut total = Rc::new(RefCell::new(Some(0)));
    for r#mod in vec![(*r#fn.borrow().as_ref().unwrap()).clone(), 2].iter().copied() {
        { let __rhs = r#mod; let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    let mut branch = Rc::new(RefCell::new(Some(Branch { r#else: total.clone(), ..Default::default() })));
    return (*(*branch.borrow().as_ref().unwrap()).r#else.borrow().as_ref().unwrap());
}

fn main() {
    println!("{}", format!("{}", use_keyword_names(Rc::new(RefCell::new(Some(3))))));
}