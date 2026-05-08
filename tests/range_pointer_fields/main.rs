use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct node {
    pub value: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.borrow().as_ref().unwrap()))
    }
}


pub fn read(n: Rc<RefCell<Option<node>>>) -> Rc<RefCell<Option<i32>>> {

    if (*n.borrow()).is_none() {
        return Rc::new(RefCell::new(Some(-1)));
    }
    return Rc::new(RefCell::new(Some((*(*n.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()).clone())));
}

fn main() {
    let mut nodes = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(node { value: Rc::new(RefCell::new(Some(1))), ..Default::default() }))), Rc::new(RefCell::new(Some(node { value: Rc::new(RefCell::new(Some(3))), ..Default::default() })))])));
    let mut sum = Rc::new(RefCell::new(Some(0)));
    { let __range_holder = nodes.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for n in __range_values.iter() {
        if (*n.borrow()).is_some() {
        { let mut guard = sum.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + (*(*n.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())); };
        println!("{}", (*read((*n).clone()).borrow().as_ref().unwrap()));
    }
    } }
    println!("{}", { let __v = (*sum.borrow().as_ref().unwrap()).clone(); __v });
}