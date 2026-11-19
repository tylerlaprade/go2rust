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
    pub fn link(&self) -> Rc<RefCell<Option<Node>>> {
        self.next.clone()
    }
}

fn main() {
    let mut a = Rc::new(RefCell::new(Some(Node { value: Rc::new(RefCell::new(Some(1))), ..Default::default() })));
    let mut b = a.clone();
    { let new_val = Rc::new(RefCell::new(None)); b = new_val; };
    println!("{} {}", format!("{}", { let __nil_result = (*a.borrow()).is_some(); __nil_result }), format!("{}", { let __nil_result = (*b.borrow()).is_none(); __nil_result }));

    let mut shared = Rc::new(RefCell::new(Some(Node { value: Rc::new(RefCell::new(Some(2))), ..Default::default() })));
    let mut first = Rc::new(RefCell::new(Some(Node { next: shared.clone(), ..Default::default() })));
    let mut second = Rc::new(RefCell::new(Some(Node { next: shared.clone(), ..Default::default() })));
    { let new_val = Rc::new(RefCell::new(None)); (*first.borrow_mut().as_mut().unwrap()).next = new_val; };
    println!("{} {}", format!("{}", { let __nil_target = (*first.borrow().as_ref().unwrap()).next.clone(); let __nil_result = (*__nil_target.borrow()).is_none(); __nil_result }), format!("{}", { let __nil_target = (*second.borrow().as_ref().unwrap()).next.clone(); let __nil_result = (*__nil_target.borrow()).is_some(); __nil_result }));

    let mut c = Rc::new(RefCell::new(Some(Node { next: Rc::new(RefCell::new(Some(Node { value: Rc::new(RefCell::new(Some(3))), ..Default::default() }))).clone(), ..Default::default() })));
    let mut p = (*c.borrow().as_ref().unwrap()).link();
    { let new_val = Rc::new(RefCell::new(None)); p = new_val; };
    println!("{} {}", format!("{}", { let __nil_target = (*c.borrow().as_ref().unwrap()).next.clone(); let __nil_result = (*__nil_target.borrow()).is_some(); __nil_result }), format!("{}", { let __nil_result = (*p.borrow()).is_none(); __nil_result }));
}