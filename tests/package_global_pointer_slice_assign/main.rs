use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Comment {
    pub text: Rc<RefCell<Option<String>>>,
}

impl Comment {
    pub fn __go_value_clone(&self) -> Self {
        Self { text: { let __guard = self.text.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Comment {
    fn default() -> Self {
        Self { text: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.text.borrow().as_ref().unwrap()))
    }
}


pub(crate) struct GoGlobal<T> {
    value: std::cell::UnsafeCell<Option<T>>,
}
unsafe impl<T> Sync for GoGlobal<T> {}
impl<T> GoGlobal<T> {
    pub(crate) const fn new() -> Self {
        Self { value: std::cell::UnsafeCell::new(None) }
    }
    pub(crate) fn borrow(&'static self) -> &'static Option<T> {
        unsafe { &*self.value.get() }
    }
    pub(crate) fn borrow_mut(&'static self) -> &'static mut Option<T> {
        unsafe { &mut *self.value.get() }
    }
    pub(crate) fn clone(&'static self) -> std::rc::Rc<std::cell::RefCell<Option<T>>> where T: Clone {
        std::rc::Rc::new(std::cell::RefCell::new(self.borrow().clone()))
    }
}

pub(crate) static separator: GoGlobal<Rc<RefCell<Option<Comment>>>> = GoGlobal::new();


fn __go_init_globals() {
    *separator.borrow_mut() = Some(Rc::new(RefCell::new(None)));
    *separator.borrow_mut() = Some(Rc::new(RefCell::new(Some(Comment { text: Rc::new(RefCell::new(Some("//".to_string()))), ..Default::default() }))));
}


fn main() {
    __go_init_all();
    let mut list: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Comment>>>>>>> = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(None)); (3) as usize])));
    (*list.borrow_mut().as_mut().unwrap())[(0) as usize] = Rc::new(RefCell::new(Some(Comment { text: Rc::new(RefCell::new(Some("a".to_string()))), ..Default::default() })));
    (*list.borrow_mut().as_mut().unwrap())[(1) as usize] = (*separator.borrow().as_ref().unwrap()).clone();
    (*list.borrow_mut().as_mut().unwrap())[(2) as usize] = Rc::new(RefCell::new(Some(Comment { text: Rc::new(RefCell::new(Some("b".to_string()))), ..Default::default() })));
    { let __range_holder = list.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for c in __range_values.iter() {
        println!("{}", format!("{}", (*(*c.borrow().as_ref().unwrap()).text.borrow().as_ref().unwrap()).clone()));
    } }
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
