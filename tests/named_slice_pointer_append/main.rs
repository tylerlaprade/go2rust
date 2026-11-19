use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

fn format_slice<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_slice_wrapped<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Rc<RefCell<Option<T>>>]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.borrow();
            match inner.as_ref() {
                Some(value) => format!("&{}", value),
                None => "<nil>".to_string(),
            }
        }).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct node {
    pub value: Rc<RefCell<Option<i32>>>,
}

impl node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            value: __go_clone_0_0,
        }
    }
}


impl Default for node {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            value: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone, Default)]
pub struct nodes(pub Rc<RefCell<Option<Vec<Rc<RefCell<Option<node>>>>>>>);

impl Display for nodes {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice_wrapped(&self.0))
    }
}


pub fn add(mut xs: Rc<RefCell<Option<nodes>>>, n: Rc<RefCell<Option<node>>>) -> Rc<RefCell<Option<nodes>>> {
    { let new_val = { let __base = { let __named_slice = (*xs.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __base_guard = __base.borrow(); let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); drop(__base_guard); __values.push(n.clone()); Rc::new(RefCell::new(Some(nodes(Rc::new(RefCell::new(Some(__values))))))) }; let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *xs.borrow_mut() = __moved_val; };
    xs.clone()
}

pub fn keep(xs: Rc<RefCell<Option<nodes>>>) -> Rc<RefCell<Option<nodes>>> {
    let mut kept: Rc<RefCell<Option<nodes>>> = Rc::new(RefCell::new(Some(Default::default())));
    { let __range_holder = { let __named_slice = (*xs.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for n in __range_values.iter() {
        { let new_val = { let __base = { let __named_slice = (*kept.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __base_guard = __base.borrow(); let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); drop(__base_guard); __values.push(n.clone()); Rc::new(RefCell::new(Some(nodes(Rc::new(RefCell::new(Some(__values))))))) }; let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *kept.borrow_mut() = __moved_val; };
    } }
    return kept.clone();
}

fn main() {
    let mut xs: Rc<RefCell<Option<nodes>>> = Rc::new(RefCell::new(Some(Default::default())));
    let mut n = Rc::new(RefCell::new(Some(node { value: Rc::new(RefCell::new(Some(7))), ..Default::default() })));
    { let new_val = add(Rc::new(RefCell::new(Some({ let __arg_holder = xs.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), n.clone()); let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *xs.borrow_mut() = __moved_val; };
    println!("{}", format!("{}", { let __slice_holder = { let __named_slice = (*xs.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }));
    println!("{}", format!("{}", { let __slice_holder = { let __named_slice = (*keep(Rc::new(RefCell::new(Some({ let __arg_holder = xs.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() })))).borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }));
}