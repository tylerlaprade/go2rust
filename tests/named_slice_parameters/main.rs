use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Numbers(pub Rc<RefCell<Option<Vec<i32>>>>);

impl Display for Numbers {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().borrow().as_ref().unwrap()))
    }
}


impl Numbers {
    pub fn string(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("Numbers({})", { let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }))))
    }
}

pub fn total(ns: Rc<RefCell<Option<Numbers>>>) -> i32 {
    let mut sum = Rc::new(RefCell::new(Some(0)));
    { let __range_holder = { let __named_slice = (*ns.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for n in __range_values.iter().copied() {
        { let __rhs = n; let mut guard = sum.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    return (*sum.borrow().as_ref().unwrap());
}

pub fn grow(mut ns: Rc<RefCell<Option<Numbers>>>) -> Rc<RefCell<Option<Numbers>>> {
    { let new_val = { let __base = { let __named_slice = (*ns.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __base_guard = __base.borrow(); let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); drop(__base_guard); __values.push(4); Rc::new(RefCell::new(Some(Numbers(Rc::new(RefCell::new(Some(__values))))))) }; let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *ns.borrow_mut() = __moved_val; };
    ns.clone()
}

pub fn merge(a: Rc<RefCell<Option<Numbers>>>, b: Rc<RefCell<Option<Numbers>>>) -> Rc<RefCell<Option<Numbers>>> {
    { let __base = { let __named_slice = (*a.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __base_guard = __base.borrow(); let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); drop(__base_guard); let __src = { let __named_slice = (*b.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __src_guard = __src.borrow(); if let Some(__src_values) = __src_guard.as_ref() { __values.extend(__src_values.iter().cloned()); }; Rc::new(RefCell::new(Some(Numbers(Rc::new(RefCell::new(Some(__values))))))) }
}

fn main() {
    let mut nums = Rc::new(RefCell::new(Some(Numbers(Rc::new(RefCell::new(Some(vec![1, 2, 3])))))));
    let mut more = Rc::new(RefCell::new(Some(Numbers(Rc::new(RefCell::new(Some(vec![5, 6])))))));
    let mut grown = grow(Rc::new(RefCell::new(Some((*nums.borrow().as_ref().unwrap()).clone()))));
    let mut merged = merge(Rc::new(RefCell::new(Some((*nums.borrow().as_ref().unwrap()).clone()))), Rc::new(RefCell::new(Some((*more.borrow().as_ref().unwrap()).clone()))));
    println!("{} {} {} {} {}", format!("{}", total(Rc::new(RefCell::new(Some((*nums.borrow().as_ref().unwrap()).clone()))))), format!("{}", total(Rc::new(RefCell::new(Some((*grown.borrow().as_ref().unwrap()).clone()))))), format!("{}", { let __slice_holder = { let __named_slice = (*merged.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }), format!("{}", { let __seq_holder = { let __named_slice = (*merged.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(4) as usize].clone() }), format!("{}", (*(*nums.borrow().as_ref().unwrap()).string().borrow().as_ref().unwrap())));
}