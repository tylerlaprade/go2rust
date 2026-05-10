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

#[derive(Debug, Clone, Default)]
pub struct loadError {
    pub import_stack: Rc<RefCell<Option<Vec<String>>>>,
}

impl std::fmt::Display for loadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice(&self.import_stack))
    }
}


fn main() {
    let mut err = Rc::new(RefCell::new(Some(loadError { import_stack: Rc::new(RefCell::new(Some(vec!["root".to_string(), "dep".to_string()]))), ..Default::default() })));
    let mut msg = Rc::new(RefCell::new(Some("import cycle not allowed".to_string())));
    if ((*(*err.borrow().as_ref().unwrap()).import_stack.borrow().as_ref().unwrap()).len() as i32) != (0 as i32) {
        { (*msg.borrow_mut().as_mut().unwrap()).push_str(&{ let __s = Rc::new(RefCell::new(Some(format!(": import stack: {}", format_slice_values(&(*(*err.borrow().as_ref().unwrap()).import_stack.borrow().as_ref().unwrap())))))); let __value = (*__s.borrow().as_ref().unwrap()).clone(); __value }); };
    }
    println!("{}", { let __v = (*msg.borrow().as_ref().unwrap()).clone(); __v });
}