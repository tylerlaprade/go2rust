use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

fn format_slice<T>(slice: &Rc<RefCell<Option<Vec<T>>>>) -> String 
where
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| v.to_string()).collect();
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

pub trait List: std::fmt::Display {
    fn valid(&self, index: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<bool>>>;
    fn label(&self, index: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>;
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
    pub fn valid(&mut self, index: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<bool>>> {
        return Rc::new(RefCell::new(Some((*index.borrow().as_ref().unwrap()) >= 0 && ((*index.borrow().as_ref().unwrap()) as i32) < ((*self.labels.borrow().as_ref().unwrap()).len() as i32))));
    }

    pub fn label(&mut self, index: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {
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
}

fn main() {
    let mut l = Rc::new(RefCell::new(Some(list { labels: Rc::new(RefCell::new(Some(vec![3]))), ..Default::default() })));
    let mut valid = (*l.borrow_mut().as_mut().unwrap()).valid(Rc::new(RefCell::new(Some(0))));
    let mut label = (*l.borrow_mut().as_mut().unwrap()).label(Rc::new(RefCell::new(Some(0))));
    println!("{} {}", { let __v = (*valid.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*label.borrow().as_ref().unwrap()).clone(); __v });
}