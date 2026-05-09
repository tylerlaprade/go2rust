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

pub trait List: std::fmt::Display + Any {
    fn __go_clone_box(&self) -> Box<dyn List>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq(&self, other: &dyn List) -> bool;
    fn valid(&self, index: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<bool>>>;
    fn label(&self, index: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>;
}

impl Clone for Box<dyn List> {
    fn clone(&self) -> Self {
        self.__go_clone_box()
    }
}

#[derive(Debug, Clone, Default)]
pub struct list {
    pub labels: Rc<RefCell<Option<Vec<i32>>>>,
}

impl std::fmt::Display for list {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice(&self.labels))
    }
}


impl list {
    pub fn valid(&self, index: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<bool>>> {
        return Rc::new(RefCell::new(Some((*index.borrow().as_ref().unwrap()) >= 0 && ((*index.borrow().as_ref().unwrap()) as i32) < ((*self.labels.borrow().as_ref().unwrap()).len() as i32))));
    }

    pub fn label(&self, index: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {
        return Rc::new(RefCell::new(Some((*self.labels.borrow().as_ref().unwrap())[((*index.borrow().as_ref().unwrap())) as usize].clone())));
    }
}

impl List for list {
    fn valid(&self, index: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<bool>>> {
        return Rc::new(RefCell::new(Some((*index.borrow().as_ref().unwrap()) >= 0 && ((*index.borrow().as_ref().unwrap()) as i32) < ((*self.labels.borrow().as_ref().unwrap()).len() as i32))));
    }
    fn label(&self, index: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {
        return Rc::new(RefCell::new(Some((*self.labels.borrow().as_ref().unwrap())[((*index.borrow().as_ref().unwrap())) as usize].clone())));
    }
    fn __go_clone_box(&self) -> Box<dyn List> {
        Box::new(self.clone()) as Box<dyn List>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq(&self, other: &dyn List) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<list>() {
            false
        } else {
            false
        }
    }
}

fn main() {
    let mut l = Rc::new(RefCell::new(Some(list { labels: Rc::new(RefCell::new(Some(vec![3]))), ..Default::default() })));
    let mut valid = (*l.borrow_mut().as_mut().unwrap()).valid(Rc::new(RefCell::new(Some(0))));
    let mut label = (*l.borrow_mut().as_mut().unwrap()).label(Rc::new(RefCell::new(Some(0))));
    println!("{} {}", { let __v = (*valid.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*label.borrow().as_ref().unwrap()).clone(); __v });
}