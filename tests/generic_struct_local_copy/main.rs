use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Box__int {
    pub value: Rc<RefCell<Option<i64>>>,
    pub count: Rc<RefCell<Option<i64>>>,
}

impl Box__int {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.count.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            value: __go_clone_0_0,
            count: __go_clone_1_0,
        }
    }
}


impl Default for Box__int {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            value: __go_default_0_0,
            count: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Box__int {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.count.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub fn copy_local__int(input: Rc<RefCell<Option<Box__int>>>) -> Rc<RefCell<Option<Box__int>>> {
    let mut first = { let __owned = input.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
    let mut second = { let __owned = first.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
    { let __target = (*second.borrow().as_ref().unwrap()).count.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    return { let __owned = second.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
}

fn main() {
    let mut original = Rc::new(RefCell::new(Some(Box__int { value: Rc::new(RefCell::new(Some(7))), count: Rc::new(RefCell::new(Some(10))), ..Default::default() })));
    let mut copied = copy_local__int(Rc::new(RefCell::new(Some({ let __arg_holder = original.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))));
    println!("{} {} {} {}", format!("{}", (*(*original.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())), format!("{}", (*(*original.borrow().as_ref().unwrap()).count.borrow().as_ref().unwrap())), format!("{}", (*(*copied.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())), format!("{}", (*(*copied.borrow().as_ref().unwrap()).count.borrow().as_ref().unwrap())));

    let mut explicit = Rc::new(RefCell::new(Some(Box__int { value: Rc::new(RefCell::new(Some(8))), count: Rc::new(RefCell::new(Some(20))), ..Default::default() })));
    let mut explicitCopy = copy_local__int(Rc::new(RefCell::new(Some({ let __arg_holder = explicit.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))));
    println!("{} {} {} {}", format!("{}", (*(*explicit.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())), format!("{}", (*(*explicit.borrow().as_ref().unwrap()).count.borrow().as_ref().unwrap())), format!("{}", (*(*explicitCopy.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())), format!("{}", (*(*explicitCopy.borrow().as_ref().unwrap()).count.borrow().as_ref().unwrap())));
}