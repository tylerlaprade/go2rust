use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Branch {
    pub r#else: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.r#else.borrow().as_ref().unwrap()))
    }
}


pub fn use_keyword_names(r#fn: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    let mut total = Rc::new(RefCell::new(Some(0)));
    for r#mod in vec![(*r#fn.borrow().as_ref().unwrap()).clone(), 2].iter().copied() {
        { let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + r#mod); };
    }
    let mut branch = Rc::new(RefCell::new(Some(Branch { r#else: total.clone(), ..Default::default() })));
    return Rc::new(RefCell::new(Some((*(*branch.borrow().as_ref().unwrap()).r#else.borrow().as_ref().unwrap()))));
}

fn main() {
    println!("{}", (*use_keyword_names(Rc::new(RefCell::new(Some(3)))).borrow().as_ref().unwrap()));
}