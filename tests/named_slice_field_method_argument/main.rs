use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct numbers(pub Rc<RefCell<Option<Vec<i32>>>>);

impl Display for numbers {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct holder {
    pub terms: Rc<RefCell<Option<numbers>>>,
}

impl holder {
    pub fn __go_value_clone(&self) -> Self {
        Self { terms: self.terms.clone() }
    }
}


impl Default for holder {
    fn default() -> Self {
        Self { terms: Rc::new(RefCell::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.terms.borrow().as_ref().unwrap()))
    }
}


impl numbers {
    pub fn string(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("numbers({})", { let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }))))
    }

    pub fn intersect(&self, other: Rc<RefCell<Option<numbers>>>) -> Rc<RefCell<Option<numbers>>> {
        let mut out = Rc::new(RefCell::new(Some(numbers(Rc::new(RefCell::new(Some(vec![])))))));
        { let __range_holder = self.0.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for left in __range_values.iter().copied() {
        { let __range_holder = { let __named_slice = (*other.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for right in __range_values.iter().copied() {
        if left == right {
        { let new_val = { let __base = { let __named_slice = (*out.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __base_guard = __base.borrow(); let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); drop(__base_guard); __values.push(left); Rc::new(RefCell::new(Some(numbers(Rc::new(RefCell::new(Some(__values))))))) }; let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *out.borrow_mut() = __moved_val; };
    }
    } }
    } }
        return out.clone();
    }
}

pub fn combine(a: Rc<RefCell<Option<holder>>>, b: Rc<RefCell<Option<holder>>>) {
    { let new_val = (*(*a.borrow().as_ref().unwrap()).terms.borrow().as_ref().unwrap()).intersect({ let __field = (*b.borrow().as_ref().unwrap()).terms.clone(); __field }); let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *(*a.borrow().as_ref().unwrap()).terms.borrow_mut() = __moved_val; };
}

pub fn copied_len(src: Rc<RefCell<Option<holder>>>) -> i32 {
    let mut terms: Rc<RefCell<Option<numbers>>> = Rc::new(RefCell::new(Some(Default::default())));
    { let new_val = { let __selector_holder = (*src.borrow().as_ref().unwrap()).terms.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *terms.borrow_mut() = Some(new_val); };
    return { let __slice_holder = { let __named_slice = (*terms.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32;
}

fn main() {
    let mut a = Rc::new(RefCell::new(Some(holder { terms: Rc::new(RefCell::new(Some(numbers(Rc::new(RefCell::new(Some(vec![1, 2, 3]))))))), ..Default::default() })));
    let mut b = Rc::new(RefCell::new(Some(holder { terms: Rc::new(RefCell::new(Some(numbers(Rc::new(RefCell::new(Some(vec![2, 4]))))))), ..Default::default() })));
    combine(a.clone(), b.clone());
    println!("{} {}", format!("{}", { let __slice_holder = { let __named_slice = (*(*a.borrow().as_ref().unwrap()).terms.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }), format!("{}", { let __seq_holder = { let __named_slice = (*(*a.borrow().as_ref().unwrap()).terms.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }));
    println!("{}", format!("{}", copied_len(b.clone())));
}