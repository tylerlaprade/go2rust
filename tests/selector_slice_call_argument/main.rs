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
pub struct Package {
    pub dir: Rc<RefCell<Option<String>>>,
    pub go_files: Rc<RefCell<Option<Vec<String>>>>,
    pub cgo_files: Rc<RefCell<Option<Vec<String>>>>,
}

impl Package {
    pub fn __go_value_clone(&self) -> Self {
        Self { dir: { let __guard = self.dir.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, go_files: self.go_files.clone(), cgo_files: self.cgo_files.clone() }
    }
}


impl Default for Package {
    fn default() -> Self {
        Self { dir: Rc::new(RefCell::new(Some(String::new()))), go_files: Rc::new(RefCell::new(None)), cgo_files: Rc::new(RefCell::new(None)) }
    }
}

impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.dir.borrow().as_ref().unwrap()), format_slice(&self.go_files), format_slice(&self.cgo_files))
    }
}


pub fn abs_join(dir: Rc<RefCell<Option<String>>>, fileses: Rc<RefCell<Option<Vec<Vec<String>>>>>) -> Rc<RefCell<Option<Vec<String>>>> {

    return Rc::new(RefCell::new(Some(vec![(*dir.borrow().as_ref().unwrap()).clone(), (*fileses.borrow().as_ref().unwrap())[(0) as usize].clone()[(0) as usize].clone(), (*fileses.borrow().as_ref().unwrap())[(1) as usize].clone()[(0) as usize].clone()])));
}

fn main() {
    let mut p = Rc::new(RefCell::new(Some(Package { dir: Rc::new(RefCell::new(Some("root".to_string()))), go_files: Rc::new(RefCell::new(Some(vec!["a.go".to_string()]))), cgo_files: Rc::new(RefCell::new(Some(vec!["c.go".to_string()]))), ..Default::default() })));
    let mut files = abs_join((*p.borrow().as_ref().unwrap()).dir.clone(), Rc::new(RefCell::new(Some(vec![{ let __selector_holder = (*p.borrow().as_ref().unwrap()).go_files.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, { let __selector_holder = (*p.borrow().as_ref().unwrap()).cgo_files.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }]))));
    { let new_val = "changed".to_string(); *(*p.borrow().as_ref().unwrap()).dir.borrow_mut() = Some(new_val); };
    (*(*p.borrow().as_ref().unwrap()).go_files.borrow_mut().as_mut().unwrap())[(0) as usize] = "b.go".to_string();
    println!("{}", format!("{}", (*files.borrow().as_ref().unwrap())[(0) as usize].clone()));
    println!("{}", format!("{}", (*files.borrow().as_ref().unwrap())[(1) as usize].clone()));
    println!("{}", format!("{}", (*files.borrow().as_ref().unwrap())[(2) as usize].clone()));
}