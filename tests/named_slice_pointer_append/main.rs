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


#[derive(Debug, Clone, Default)]
pub struct nodes(pub Rc<RefCell<Option<Vec<Rc<RefCell<Option<node>>>>>>>);


pub fn add(xs: Rc<RefCell<Option<nodes>>>, n: Rc<RefCell<Option<node>>>) -> Rc<RefCell<Option<nodes>>> {

    { let new_val = { let __base = { let __named_slice = (*xs.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __base_guard = __base.borrow(); let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); drop(__base_guard); __values.push(n.clone()); Rc::new(RefCell::new(Some(nodes(Rc::new(RefCell::new(Some(__values))))))) }; let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *xs.borrow_mut() = __moved_val; };
    return xs.clone();
}

pub fn keep(xs: Rc<RefCell<Option<nodes>>>) -> Rc<RefCell<Option<nodes>>> {

    let mut kept: Rc<RefCell<Option<nodes>>> = Rc::new(RefCell::new(Some(Default::default())));
    { let __range_holder = { let __named_slice = (*xs.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for n in __range_values.iter() {
        { let new_val = { let __base = { let __named_slice = (*kept.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __base_guard = __base.borrow(); let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); drop(__base_guard); __values.push(n.clone()); Rc::new(RefCell::new(Some(nodes(Rc::new(RefCell::new(Some(__values))))))) }; let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *kept.borrow_mut() = __moved_val; };
    } }
    return kept.clone();
}

fn main() {
    let mut xs: Rc<RefCell<Option<nodes>>> = Rc::new(RefCell::new(Some(Default::default())));
    let mut n = Rc::new(RefCell::new(Some(node { value: Rc::new(RefCell::new(Some(7))), ..Default::default() })));
    { let new_val = add(Rc::new(RefCell::new(Some((*xs.borrow().as_ref().unwrap()).clone()))), n.clone()); let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *xs.borrow_mut() = __moved_val; };
    println!("{}", { let __slice_holder = { let __named_slice = (*xs.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) });
    println!("{}", { let __slice_holder = { let __named_slice = (*keep(Rc::new(RefCell::new(Some((*xs.borrow().as_ref().unwrap()).clone())))).borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) });
}