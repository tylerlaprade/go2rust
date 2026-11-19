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
        let __go_clone_0_0 = { let __guard = self.verb.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = self.build_flags.clone();
        Self {
            verb: __go_clone_0_0,
            build_flags: __go_clone_1_0,
        }
    }
}


impl Default for invocation {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        Self {
            verb: __go_default_0_0,
            build_flags: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for invocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.verb.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.build_flags));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl invocation {
    pub fn run(&self) -> Rc<RefCell<Option<Vec<String>>>> {
        let mut goArgs = Rc::new(RefCell::new(Some(vec![{ let __selector_holder = self.verb.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }])));
        { let new_val = { let __append_target = goArgs.clone(); let __append_values: Vec<_> = { let __slice_holder = self.build_flags.clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned().collect(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).extend(__append_values); __append_target.clone() }; goArgs = new_val; };
        return goArgs.clone();
    }
}

fn main() {
    let mut inv = Rc::new(RefCell::new(Some(invocation { verb: Rc::new(RefCell::new(Some("list".to_string()))), ..Default::default() })));
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some({ let __parts = (*(*inv.borrow().as_ref().unwrap()).run().borrow()).as_ref().cloned().unwrap_or_default(); let __sep = ",".to_string(); __parts.join(&__sep) }))).borrow().as_ref().unwrap())));
}