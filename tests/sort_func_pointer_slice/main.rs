use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Item {
    pub pos: Rc<RefCell<Option<i32>>>,
}

impl Item {
    pub fn __go_value_clone(&self) -> Self {
        Self { pos: { let __guard = self.pos.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Item {
    fn default() -> Self {
        Self { pos: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.pos.borrow().as_ref().unwrap()))
    }
}


impl Item {
    pub fn pos(&self) -> i32 {
        return (*self.pos.borrow().as_ref().unwrap());
    }
}

fn main() {
    let mut items = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Item { pos: Rc::new(RefCell::new(Some(3))), ..Default::default() }))), Rc::new(RefCell::new(Some(Item { pos: Rc::new(RefCell::new(Some(1))), ..Default::default() }))), Rc::new(RefCell::new(Some(Item { pos: Rc::new(RefCell::new(Some(2))), ..Default::default() })))])));
    { let __cmp_holder = Rc::new(RefCell::new(Some(Box::new(move |a: Rc<RefCell<Option<Item>>>, b: Rc<RefCell<Option<Item>>>| -> i32 {
        (*a.borrow().as_ref().unwrap()).pos() - (*b.borrow().as_ref().unwrap()).pos()
    }) as Box<dyn FnMut(Rc<RefCell<Option<Item>>>, Rc<RefCell<Option<Item>>>) -> i32>))); let mut __sort_guard = items.borrow_mut(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort_by(|__a, __b| { let __cmp = { let mut __cmp_guard = __cmp_holder.borrow_mut(); let __cmp_fn = __cmp_guard.as_mut().unwrap(); (*__cmp_fn)(__a.clone(), __b.clone()) }; let __ord = __cmp.cmp(&0); __ord }); } };
    { let __range_holder = items.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for it in __range_values.iter() {
        println!("{}", format!("{}", (*(*it.borrow().as_ref().unwrap()).pos.borrow().as_ref().unwrap())));
    } }
}