use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct node {
    pub value: Rc<RefCell<Option<i32>>>,
}

impl node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            value: __go_clone_0_0,
        }
    }
}


impl Default for node {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            value: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl node {
    pub fn same(&self, other: Rc<RefCell<Option<node>>>) -> bool {
        if false || { let __nil_result = (*other.borrow()).is_none(); __nil_result } {
        return { let __peer = other.clone(); let __peer_guard = __peer.borrow(); let __peer_ptr = __peer_guard.as_ref().map(|__v| __v as *const _ as usize); let __self_ptr = self as *const _ as usize; let __eq = __peer_ptr == Some(__self_ptr); __eq };
    }
        return (*self.value.borrow().as_ref().unwrap()) == (*(*other.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap());
    }
}

fn main() {
    let mut left = Rc::new(RefCell::new(Some(node { value: Rc::new(RefCell::new(Some(7))), ..Default::default() })));
    let mut missing: Rc<RefCell<Option<node>>> = Rc::new(RefCell::new(None));
    println!("{}", format!("{}", __go_nil_recv_node_same({ let __recv = left.clone(); __recv }, missing.clone())));
}

pub fn __go_nil_recv_node_same(n: Rc<RefCell<Option<node>>>, other: Rc<RefCell<Option<node>>>) -> bool {
    if { let __nil_result = (*n.borrow()).is_none(); __nil_result } || { let __nil_result = (*other.borrow()).is_none(); __nil_result } {
        return { let __left = n.clone(); let __right = other.clone(); let __both_nil = (*__left.borrow()).is_none() && (*__right.borrow()).is_none(); let __eq = __both_nil || Rc::ptr_eq(&__left, &__right); __eq };
    }
    return (*(*n.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()) == (*(*other.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap());
}