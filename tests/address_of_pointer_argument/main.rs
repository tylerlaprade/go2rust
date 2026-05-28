use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct info {
    pub name: Rc<RefCell<Option<String>>>,
    pub value: Rc<RefCell<Option<i32>>>,
}

impl info {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, value: { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for info {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))), value: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for info {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.name.borrow().as_ref().unwrap()), (*self.value.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct table {
    pub last: Rc<RefCell<Option<info>>>,
}

impl table {
    pub fn __go_value_clone(&self) -> Self {
        Self { last: self.last.clone() }
    }
}

impl std::fmt::Display for table {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.last.borrow().as_ref().unwrap()))
    }
}


impl table {
    pub fn register(&mut self, name: Rc<RefCell<Option<String>>>, ptr: Rc<RefCell<Option<info>>>) {
        { let new_val = ptr.clone(); self.last = new_val; };
        println!("{} {}", format!("{}", { let __v = (*name.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*(*ptr.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())));
    }
}

pub fn accept(ptr: Rc<RefCell<Option<info>>>) -> Rc<RefCell<Option<String>>> {
    return Rc::new(RefCell::new(Some({ let __selector_holder = (*ptr.borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
}

fn main() {
    println!("{}", format!("{}", (*accept(Rc::new(RefCell::new(Some(info { name: Rc::new(RefCell::new(Some("alpha".to_string()))), value: Rc::new(RefCell::new(Some(7 as i32))), ..Default::default() })))).borrow().as_ref().unwrap())));

    let mut t = Rc::new(RefCell::new(Some(table { last: Rc::new(RefCell::new(Some(Default::default()))) })));
    (*t.borrow_mut().as_mut().unwrap()).register(Rc::new(RefCell::new(Some("beta".to_string()))), Rc::new(RefCell::new(Some(info { name: Rc::new(RefCell::new(Some("beta".to_string()))), value: Rc::new(RefCell::new(Some(9 as i32))), ..Default::default() }))));
    println!("{}", format!("{}", (*(*(*t.borrow().as_ref().unwrap()).last.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()));
}