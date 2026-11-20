use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct node {
    pub next: Rc<RefCell<Option<node>>>,
    pub v: Rc<RefCell<Option<i64>>>,
}

impl node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.next.clone();
        let __go_clone_1_0 = { let __guard = self.v.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            next: __go_clone_0_0,
            v: __go_clone_1_0,
        }
    }
}


impl Default for node {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(None));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            next: __go_default_0_0,
            v: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.next.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.v.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl node {
    pub fn walk(&self) -> i64 {
        let mut y = Rc::new(RefCell::new(Some(self.clone())));
        let mut sum = Rc::new(RefCell::new(Some(0i64)));
        while { let __nil_result = (*y.borrow()).is_some(); __nil_result } {
        { let __rhs = (*(*y.borrow().as_ref().unwrap()).v.borrow().as_ref().unwrap()); let mut guard = sum.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = (*y.borrow().as_ref().unwrap()).next.clone(); y = new_val; };
    }
        return (*sum.borrow().as_ref().unwrap());
    }
}

fn main() {
    let mut c = Rc::new(RefCell::new(Some(node { v: Rc::new(RefCell::new(Some(3))), ..Default::default() })));
    let mut b = Rc::new(RefCell::new(Some(node { v: Rc::new(RefCell::new(Some(2))), next: c.clone(), ..Default::default() })));
    let mut a = Rc::new(RefCell::new(Some(node { v: Rc::new(RefCell::new(Some(1))), next: b.clone(), ..Default::default() })));
    {
            let __go_print_arg_0 = format!("{}", (*a.borrow().as_ref().unwrap()).walk());
            eprintln!("{}", __go_print_arg_0)
        };
}