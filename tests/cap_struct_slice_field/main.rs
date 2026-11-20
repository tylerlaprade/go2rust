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
pub struct bucket {
    pub values: Rc<RefCell<Option<Vec<i64>>>>,
}

impl bucket {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.values.clone();
        Self {
            values: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for bucket {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_slice(&self.values));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl bucket {
    pub fn has_room(&self) -> bool {
        (({ let __cap_target = { let __field = self.values.clone(); __field }; let __cap_guard = __cap_target.borrow(); __cap_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) }) as i64) >= (3 as i64)
    }
}

fn main() {
    let mut b = Rc::new(RefCell::new(Some(bucket { values: Rc::new(RefCell::new(Some({ let mut v = Vec::with_capacity((3) as usize); v.resize((1) as usize, 0); v }))), ..Default::default() })));
    println!("{}", format!("{}", ({ let __len_target = { let __field = (*b.borrow().as_ref().unwrap()).values.clone(); __field }; let __len_guard = __len_target.borrow(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })));
    println!("{}", format!("{}", ({ let __cap_target = { let __field = (*b.borrow().as_ref().unwrap()).values.clone(); __field }; let __cap_guard = __cap_target.borrow(); __cap_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) })));
    println!("{}", format!("{}", (*b.borrow().as_ref().unwrap()).has_room()));
}