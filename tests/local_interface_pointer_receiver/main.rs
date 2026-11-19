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
    fn __go_clone_box_list(&self) -> Box<dyn List>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_list(&self, other: &dyn List) -> bool;
    fn valid(&self, index: Rc<RefCell<Option<i32>>>) -> bool;
    fn label(&self, index: Rc<RefCell<Option<i32>>>) -> i32;
}

impl Clone for Box<dyn List> {
    fn clone(&self) -> Self {
        List::__go_clone_box_list(self.as_ref())
    }
}

#[derive(Debug, Clone, Default)]
pub struct list {
    pub labels: Rc<RefCell<Option<Vec<i32>>>>,
}

impl list {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.labels.clone();
        Self {
            labels: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for list {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_slice(&self.labels));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl list {
    pub fn valid(&self, index: Rc<RefCell<Option<i32>>>) -> bool {
        (*index.borrow().as_ref().unwrap()) >= 0 && ((*index.borrow().as_ref().unwrap()) as i32) < (({ let __len_target = { let __field = self.labels.clone(); __field }; let __len_guard = __len_target.borrow(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)
    }

    pub fn label(&self, index: Rc<RefCell<Option<i32>>>) -> i32 {
        (*self.labels.borrow().as_ref().unwrap())[((*index.borrow().as_ref().unwrap())) as usize].clone()
    }
}

impl List for list {
    fn label(&self, index: Rc<RefCell<Option<i32>>>) -> i32 {
        list::label(self, index)
    }
    fn valid(&self, index: Rc<RefCell<Option<i32>>>) -> bool {
        list::valid(self, index)
    }
    fn __go_clone_box_list(&self) -> Box<dyn List> {
        Box::new(self.clone()) as Box<dyn List>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_list(&self, other: &dyn List) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<list>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct listPtr(pub Rc<RefCell<Option<list>>>);

impl std::fmt::Display for listPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl List for listPtr {
    fn label(&self, index: Rc<RefCell<Option<i32>>>) -> i32 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        list::label(__recv, index)
    }
    fn valid(&self, index: Rc<RefCell<Option<i32>>>) -> bool {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        list::valid(__recv, index)
    }
    fn __go_clone_box_list(&self) -> Box<dyn List> {
        Box::new(self.clone()) as Box<dyn List>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_list(&self, other: &dyn List) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<listPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

fn main() {
    let mut l = Rc::new(RefCell::new(Some(list { labels: Rc::new(RefCell::new(Some(vec![3]))), ..Default::default() })));
    let mut valid = (*l.borrow().as_ref().unwrap()).valid(Rc::new(RefCell::new(Some(0))));
    let mut label = (*l.borrow().as_ref().unwrap()).label(Rc::new(RefCell::new(Some(0))));
    println!("{} {}", format!("{}", valid), format!("{}", label));
}