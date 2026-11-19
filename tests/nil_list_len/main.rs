use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Node {
    pub value: Rc<RefCell<Option<i32>>>,
    pub next: Rc<RefCell<Option<Node>>>,
}

impl Node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = self.next.clone();
        Self {
            value: __go_clone_0_0,
            next: __go_clone_1_0,
        }
    }
}


impl Default for Node {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        Self {
            value: __go_default_0_0,
            next: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.next.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl Node {
    pub fn len(&self) -> i32 {
        if false {
        return 0;
    }
        1 + __go_nil_recv_node_len({ let __recv = self.next.clone(); __recv })
    }
}

fn main() {
    let mut list = Rc::new(RefCell::new(Some(Node { value: Rc::new(RefCell::new(Some(1))), next: Rc::new(RefCell::new(Some(Node { value: Rc::new(RefCell::new(Some(2))), next: Default::default(), ..Default::default() }))).clone(), ..Default::default() })));
    println!("{}", format!("{}", __go_nil_recv_node_len({ let __recv = list.clone(); __recv })));
    let mut empty: Rc<RefCell<Option<Node>>> = Rc::new(RefCell::new(None));
    println!("{}", format!("{}", __go_nil_recv_node_len({ let __recv = empty.clone(); __recv })));
}

pub fn __go_nil_recv_node_len(n: Rc<RefCell<Option<Node>>>) -> i32 {
    if { let __nil_result = (*n.borrow()).is_none(); __nil_result } {
        return 0;
    }
    1 + __go_nil_recv_node_len({ let __recv = (*n.borrow().as_ref().unwrap()).next.clone(); __recv })
}