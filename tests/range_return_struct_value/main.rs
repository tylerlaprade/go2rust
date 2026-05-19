use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct item {
    pub name: Rc<RefCell<Option<String>>>,
}

impl item {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for item {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct scorer {
}

impl scorer {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for scorer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


#[derive(Debug, Clone, Default)]
pub struct holder {
    pub s: Rc<RefCell<Option<scorer>>>,
}

impl holder {
    pub fn __go_value_clone(&self) -> Self {
        Self { s: self.s.clone() }
    }
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.s.borrow().as_ref().unwrap()))
    }
}


impl scorer {
    pub fn score(&self, item: Rc<RefCell<Option<item>>>) -> Rc<RefCell<Option<i32>>> {
        return Rc::new(RefCell::new(Some((*(*item.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).len() as i32)));
    }
}

pub fn first(items: Rc<RefCell<Option<Vec<item>>>>) -> Rc<RefCell<Option<item>>> {

    { let __range_holder = items.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for item in __range_values.iter() {
        return Rc::new(RefCell::new(Some(item.clone())));
    } }
    return Rc::new(RefCell::new(Some(item { name: Rc::new(RefCell::new(Some(String::new()))) })));
}

pub fn score(item: Rc<RefCell<Option<item>>>) -> Rc<RefCell<Option<i32>>> {

    return Rc::new(RefCell::new(Some((*(*item.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).len() as i32)));
}

pub fn total_score(items: Rc<RefCell<Option<Vec<item>>>>) -> Rc<RefCell<Option<i32>>> {

    let mut total = Rc::new(RefCell::new(Some(0)));
    { let __range_holder = items.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for item in __range_values.iter() {
        { let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + (*score(Rc::new(RefCell::new(Some(item.clone())))).borrow().as_ref().unwrap())); };
    } }
    return total.clone();
}

pub fn total_method_score(h: Rc<RefCell<Option<holder>>>, items: Rc<RefCell<Option<Vec<item>>>>) -> Rc<RefCell<Option<i32>>> {

    let mut total = Rc::new(RefCell::new(Some(0)));
    { let __range_holder = items.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for item in __range_values.iter() {
        { let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + (*(*(*h.borrow().as_ref().unwrap()).s.borrow_mut().as_mut().unwrap()).score(Rc::new(RefCell::new(Some((*item).clone())))).borrow().as_ref().unwrap())); };
    } }
    return total.clone();
}

fn main() {
    let mut items = Rc::new(RefCell::new(Some(vec![item { name: Rc::new(RefCell::new(Some("alpha".to_string()))), ..Default::default() }, item { name: Rc::new(RefCell::new(Some("beta".to_string()))), ..Default::default() }])));
    let mut h = Rc::new(RefCell::new(Some(holder { s: Rc::new(RefCell::new(Some(scorer {  }))).clone(), ..Default::default() })));
    println!("{}", format!("{}", (*(*first(items.clone()).borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*total_score(items.clone()).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*total_method_score(h.clone(), items.clone()).borrow().as_ref().unwrap())));
}