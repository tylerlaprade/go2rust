use std::any::Any;
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

pub trait Node: std::fmt::Display + Any {
    fn __go_clone_box_node(&self) -> Box<dyn Node>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_node(&self, other: &dyn Node) -> bool;
    fn tag(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn Node> {
    fn clone(&self) -> Self {
        self.__go_clone_box_node()
    }
}

#[derive(Debug, Clone)]
pub struct Decl {
    pub tag: Rc<RefCell<Option<String>>>,
    pub items: Rc<RefCell<Option<Vec<i32>>>>,
}

impl Decl {
    pub fn __go_value_clone(&self) -> Self {
        Self { tag: { let __guard = self.tag.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, items: self.items.clone() }
    }
}


impl Default for Decl {
    fn default() -> Self {
        Self { tag: Rc::new(RefCell::new(Some(String::new()))), items: Rc::new(RefCell::new(None)) }
    }
}

impl std::fmt::Display for Decl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.tag.borrow().as_ref().unwrap()), format_slice(&self.items))
    }
}


impl Decl {
    pub fn tag(&self) -> Rc<RefCell<Option<String>>> {
        return self.tag.clone();
    }
}

impl Node for Decl {
    fn tag(&self) -> Rc<RefCell<Option<String>>> {
        return self.tag.clone();
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node> {
        Box::new(self.clone()) as Box<dyn Node>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &dyn Node) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Decl>() {
            false
        } else {
            false
        }
    }
}

pub fn process(n: &dyn Node) {
    let (mut d, mut ok) = ({
        let any_val = n.__go_as_any();
        if let Some(typed_val) = any_val.downcast_ref::<Decl>() {
            (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
        } else {
            (Rc::new(RefCell::new(None::<Decl>)), Rc::new(RefCell::new(Some(false))))
        }
    });
    if !(*ok.borrow().as_ref().unwrap()) {
        return;
    }
    { let new_val = { let __append_target = (*d.borrow().as_ref().unwrap()).items.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(99); __append_target.clone() }; (*d.borrow_mut().as_mut().unwrap()).items = new_val; };
}

fn main() {
    let mut a = Rc::new(RefCell::new(Some(Decl { tag: Rc::new(RefCell::new(Some("a".to_string()))), items: Rc::new(RefCell::new(Some(vec![1, 2]))), ..Default::default() })));
    let mut b = Rc::new(RefCell::new(Some(Decl { tag: Rc::new(RefCell::new(Some("b".to_string()))), items: Rc::new(RefCell::new(Some(vec![3]))), ..Default::default() })));
    process(a.borrow().as_ref().unwrap());
    process(b.borrow().as_ref().unwrap());
    println!("{} {}", format!("{}", (*(*a.borrow().as_ref().unwrap()).tag.borrow().as_ref().unwrap()).clone()), format!("{}", format_slice_values(&(*(*a.borrow().as_ref().unwrap()).items.borrow().as_ref().unwrap()).clone())));
    println!("{} {}", format!("{}", (*(*b.borrow().as_ref().unwrap()).tag.borrow().as_ref().unwrap()).clone()), format!("{}", format_slice_values(&(*(*b.borrow().as_ref().unwrap()).items.borrow().as_ref().unwrap()).clone())));
}