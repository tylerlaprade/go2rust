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
    pub fn swap(&self, i: Rc<RefCell<Option<i32>>>, j: Rc<RefCell<Option<i32>>>) {
        { let __tmp_0 = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[((*j.borrow().as_ref().unwrap())) as usize].clone() }; let __tmp_1 = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[((*i.borrow().as_ref().unwrap())) as usize].clone() }; (*self.0.borrow_mut().as_mut().unwrap())[((*i.borrow().as_ref().unwrap())) as usize] = __tmp_0; (*self.0.borrow_mut().as_mut().unwrap())[((*j.borrow().as_ref().unwrap())) as usize] = __tmp_1; };
    }

    pub fn at(&self, i: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some({ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[((*i.borrow().as_ref().unwrap())) as usize].clone() })));
    }
}

fn main() {
    let mut names = Rc::new(RefCell::new(Some(Names(Rc::new(RefCell::new(Some(vec!["ada".to_string(), "grace".to_string(), "lin".to_string()])))))));
    (*names.borrow().as_ref().unwrap()).swap(Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(2))));
    println!("{} {}", format!("{}", (*(*names.borrow().as_ref().unwrap()).at(Rc::new(RefCell::new(Some(0)))).borrow().as_ref().unwrap())), format!("{}", (*(*names.borrow().as_ref().unwrap()).at(Rc::new(RefCell::new(Some(2)))).borrow().as_ref().unwrap())));
}