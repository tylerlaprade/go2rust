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
pub struct Item {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Item {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for Item {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct Holder {
    pub item: Rc<RefCell<Option<Item>>>,
    pub values: Rc<RefCell<Option<Vec<i32>>>>,
}

impl Holder {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.item.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = self.values.clone();
        Self {
            item: __go_clone_0_0,
            values: __go_clone_1_0,
        }
    }
}


impl Default for Holder {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Item::default())));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        Self {
            item: __go_default_0_0,
            values: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.item.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.values));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub fn get_item(h: Rc<RefCell<Option<Holder>>>) -> Rc<RefCell<Option<Item>>> {
    return Rc::new(RefCell::new(Some({ let __selector_holder = (*h.borrow().as_ref().unwrap()).item.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = __selector_guard.as_ref().unwrap().__go_value_clone(); drop(__selector_guard); __cloned })));
}

pub fn get_values(h: Rc<RefCell<Option<Holder>>>) -> Rc<RefCell<Option<Vec<i32>>>> {
    return (*h.borrow().as_ref().unwrap()).values.clone();
}

fn main() {
    let mut h = Rc::new(RefCell::new(Some(Holder { item: Rc::new(RefCell::new(Some(Item { name: Rc::new(RefCell::new(Some("go".to_string()))), ..Default::default() }))), values: Rc::new(RefCell::new(Some(vec![2, 3]))), ..Default::default() })));

    let mut item = get_item(Rc::new(RefCell::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut values = get_values(Rc::new(RefCell::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));

    println!("{}", format!("{}", (*(*item.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()));
    println!("{} {} {}", format!("{}", (*values.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*values.borrow().as_ref().unwrap())[(0) as usize].clone()), format!("{}", (*values.borrow().as_ref().unwrap())[(1) as usize].clone()));
}