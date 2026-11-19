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
        let __go_clone_0_0 = { let __guard = self.dir.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = self.go_files.clone();
        let __go_clone_2_0 = self.cgo_files.clone();
        Self {
            dir: __go_clone_0_0,
            go_files: __go_clone_1_0,
            cgo_files: __go_clone_2_0,
        }
    }
}


impl Default for Package {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        let __go_default_2_0 = Rc::new(RefCell::new(None));
        Self {
            dir: __go_default_0_0,
            go_files: __go_default_1_0,
            cgo_files: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.dir.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.go_files));
        let __go_fmt_2 = format!("{}", format_slice(&self.cgo_files));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


pub fn abs_join(dir: Rc<RefCell<Option<String>>>, fileses: Rc<RefCell<Option<Vec<Vec<String>>>>>) -> Rc<RefCell<Option<Vec<String>>>> {
    Rc::new(RefCell::new(Some(vec![(*dir.borrow().as_ref().unwrap()).clone(), (*fileses.borrow().as_ref().unwrap())[(0) as usize].clone()[(0) as usize].clone(), (*fileses.borrow().as_ref().unwrap())[(1) as usize].clone()[(0) as usize].clone()])))
}

fn main() {
    let mut p = Rc::new(RefCell::new(Some(Package { dir: Rc::new(RefCell::new(Some("root".to_string()))), go_files: Rc::new(RefCell::new(Some(vec!["a.go".to_string()]))), cgo_files: Rc::new(RefCell::new(Some(vec!["c.go".to_string()]))), ..Default::default() })));
    let mut files = abs_join(Rc::new(RefCell::new(Some({ let __selector_holder = (*p.borrow().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Rc::new(RefCell::new(Some(vec![{ let __slice_holder = (*p.borrow().as_ref().unwrap()).go_files.clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }, { let __slice_holder = (*p.borrow().as_ref().unwrap()).cgo_files.clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }]))));
    { let new_val = "changed".to_string(); *(*p.borrow().as_ref().unwrap()).dir.borrow_mut() = Some(new_val); };
    (*(*p.borrow().as_ref().unwrap()).go_files.borrow_mut().as_mut().unwrap())[(0) as usize] = "b.go".to_string();
    println!("{}", format!("{}", (*files.borrow().as_ref().unwrap())[(0) as usize].clone()));
    println!("{}", format!("{}", (*files.borrow().as_ref().unwrap())[(1) as usize].clone()));
    println!("{}", format!("{}", (*files.borrow().as_ref().unwrap())[(2) as usize].clone()));
}