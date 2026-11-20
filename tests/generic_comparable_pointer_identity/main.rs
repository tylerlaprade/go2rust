use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Var {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Var {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for Var {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub fn changed__ptr__var(r#in: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Var>>>>>>>, subst: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<Var>>>) -> Rc<RefCell<Option<Var>>>>>>>) -> bool {
    if ((*r#in.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64) == (0 as i64) {
        return false;
    }
    let mut u = { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<Var>>>) -> Rc<RefCell<Option<Var>>>> = { let mut __f_guard = subst.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<Var>>>) -> Rc<RefCell<Option<Var>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)((*r#in.borrow().as_ref().unwrap())[(0) as usize].clone()) };
    return { let __left = u.clone(); let __right = (*r#in.borrow().as_ref().unwrap())[(0) as usize].clone(); let __both_nil = (*__left.borrow()).is_none() && (*__right.borrow()).is_none(); let __eq = __both_nil || Rc::ptr_eq(&__left, &__right); !__eq };
}

fn main() {
    let mut a = Rc::new(RefCell::new(Some(Var { name: Rc::new(RefCell::new(Some("same".to_string()))), ..Default::default() })));
    let mut b = Rc::new(RefCell::new(Some(Var { name: Rc::new(RefCell::new(Some("same".to_string()))), ..Default::default() })));

    let b_closure_clone = b.clone(); println!("{}", format!("{}", changed__ptr__var(Rc::new(RefCell::new(Some(vec![a.clone()]))), Rc::new(RefCell::new(Some(Box::new(move |_: Rc<RefCell<Option<Var>>>| -> Rc<RefCell<Option<Var>>> {
        return b_closure_clone.clone();
    }) as Box<dyn FnMut(Rc<RefCell<Option<Var>>>) -> Rc<RefCell<Option<Var>>>>))))));
    let a_closure_clone = a.clone(); println!("{}", format!("{}", changed__ptr__var(Rc::new(RefCell::new(Some(vec![a_closure_clone.clone()]))), Rc::new(RefCell::new(Some({ let a_closure_clone_closure_clone = a_closure_clone.clone(); Box::new(move |_: Rc<RefCell<Option<Var>>>| -> Rc<RefCell<Option<Var>>> {
        return a_closure_clone_closure_clone.clone();
    }) as Box<dyn FnMut(Rc<RefCell<Option<Var>>>) -> Rc<RefCell<Option<Var>>>> }))))));
}