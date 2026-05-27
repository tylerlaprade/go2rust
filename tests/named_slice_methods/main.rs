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
pub struct Names(pub Rc<RefCell<Option<Vec<String>>>>);

impl Display for Names {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


impl Names {
    pub fn len(&self) -> i32 {
        { let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32
    }

    pub fn first(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some({ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() })))
    }

    pub fn join(&self) -> Rc<RefCell<Option<String>>> {
        let mut out = Rc::new(RefCell::new(Some("".to_string())));
        { let __range_holder = self.0.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, name) in __range_values.iter().enumerate() {
        if i > 0 {
        { (*out.borrow_mut().as_mut().unwrap()).push_str(&",".to_string()); };
    }
        { (*out.borrow_mut().as_mut().unwrap()).push_str(&name); };
    } }
        Rc::new(RefCell::new(Some(out.borrow().as_ref().unwrap().clone())))
    }
}

fn main() {
    let mut names = Rc::new(RefCell::new(Some(Names(Rc::new(RefCell::new(Some(vec!["ada".to_string(), "grace".to_string()])))))));
    println!("{} {}", format!("{}", "Len:".to_string()), format!("{}", (*names.borrow().as_ref().unwrap()).len()));
    println!("{} {}", format!("{}", "First:".to_string()), format!("{}", (*(*names.borrow().as_ref().unwrap()).first().borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "Join:".to_string()), format!("{}", (*(*names.borrow().as_ref().unwrap()).join().borrow().as_ref().unwrap())));
}