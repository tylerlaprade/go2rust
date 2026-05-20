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
pub struct invocation {
    pub verb: Rc<RefCell<Option<String>>>,
    pub build_flags: Rc<RefCell<Option<Vec<String>>>>,
}

impl invocation {
    pub fn __go_value_clone(&self) -> Self {
        Self { verb: { let __guard = self.verb.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, build_flags: self.build_flags.clone() }
    }
}


impl Default for invocation {
    fn default() -> Self {
        Self { verb: Rc::new(RefCell::new(Some(String::new()))), build_flags: Rc::new(RefCell::new(None)) }
    }
}

impl std::fmt::Display for invocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.verb.borrow().as_ref().unwrap()), format_slice(&self.build_flags))
    }
}


impl invocation {
    pub fn run(&self) -> Rc<RefCell<Option<Vec<String>>>> {
        let mut goArgs = Rc::new(RefCell::new(Some(vec![{ let __selector_holder = self.verb.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }])));
        { let new_val = { let __append_target = goArgs.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).extend({ let __slice_holder = self.build_flags.clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; goArgs = new_val; };
        return goArgs.clone();
    }
}

fn main() {
    let mut inv = Rc::new(RefCell::new(Some(invocation { verb: Rc::new(RefCell::new(Some("list".to_string()))), ..Default::default() })));
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some({ let __parts = (*(*inv.borrow().as_ref().unwrap()).run().borrow().as_ref().unwrap()).clone(); let __sep = ",".to_string(); __parts.join(&__sep) }))).borrow().as_ref().unwrap())));
}