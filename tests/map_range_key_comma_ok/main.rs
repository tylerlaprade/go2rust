use std::cell::{RefCell};
use std::collections::BTreeMap;
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

#[derive(Debug, Clone, Default)]
pub struct Package {
    pub errors: Rc<RefCell<Option<Vec<String>>>>,
}

impl Package {
    pub fn __go_value_clone(&self) -> Self {
        Self { errors: self.errors.clone() }
    }
}

impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice(&self.errors))
    }
}


fn main() {
    let mut pkgs = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Package>>>>::from([("pkg".to_string(), Rc::new(RefCell::new(Some(Package { ..Default::default() }))).clone())]))));
    let mut additional = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Vec<String>>>>>::from([("pkg".to_string(), Rc::new(RefCell::new(Some(vec!["missing file".to_string()]))))]))));

    for (id, errs) in { let __range_holder = additional.clone(); let __range_guard = __range_holder.borrow(); let __range_map = (*__range_guard.as_ref().unwrap()).clone(); drop(__range_guard); __range_map } {
        let (mut p, mut ok) = match (*pkgs.borrow().as_ref().unwrap()).get(&id) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), Rc::new(RefCell::new(Some(true)))), None => (Default::default(), Rc::new(RefCell::new(Some(false)))) };
    if (*ok.borrow().as_ref().unwrap()) {
        { let new_val = { let __append_target = (*p.borrow().as_ref().unwrap()).errors.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).extend((*errs.borrow().as_ref().unwrap()).clone().iter().cloned()); __append_target.clone() }; (*p.borrow_mut().as_mut().unwrap()).errors = new_val; };
    }
    }

    println!("{}", (*(*(*pkgs.borrow().as_ref().unwrap()).get(&"pkg".to_string()).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()).borrow().as_ref().unwrap()).errors.borrow().as_ref().unwrap())[(0) as usize].clone());
}