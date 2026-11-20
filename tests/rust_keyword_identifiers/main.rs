use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Branch {
    pub r#else: Rc<RefCell<Option<i64>>>,
}

impl Branch {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.r#else.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            r#else: __go_clone_0_0,
        }
    }
}


impl Default for Branch {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            r#else: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.r#else.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub fn use_keyword_names(r#fn: Rc<RefCell<Option<i64>>>) -> i64 {
    let mut total = Rc::new(RefCell::new(Some(0i64)));
    for r#mod in vec![(*r#fn.borrow().as_ref().unwrap()).clone(), 2i64].iter().copied() {
        { let __rhs = r#mod; let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    let mut branch = Rc::new(RefCell::new(Some(Branch { r#else: Rc::new(RefCell::new(Some({ let __arg_holder = total.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
    return (*(*branch.borrow().as_ref().unwrap()).r#else.borrow().as_ref().unwrap());
}

fn main() {
    println!("{}", format!("{}", use_keyword_names(Rc::new(RefCell::new(Some(3))))));
}