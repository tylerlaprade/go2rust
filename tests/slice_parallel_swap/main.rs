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
    pub fn swap(&self, i: Rc<RefCell<Option<i64>>>, j: Rc<RefCell<Option<i64>>>) {
        let mut __self = self.clone();
        {
            let __go_lhs_sequence_0 = __self.0.clone().clone();
            let __go_lhs_index_0 = ((*i.borrow().as_ref().unwrap())) as usize;
            let __go_lhs_sequence_1 = __self.0.clone().clone();
            let __go_lhs_index_1 = ((*j.borrow().as_ref().unwrap())) as usize;
            let __tmp_0 = { let __seq_holder = __self.0.clone(); let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[((*j.borrow().as_ref().unwrap())) as usize].clone() };
            let __tmp_1 = { let __seq_holder = __self.0.clone(); let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[((*i.borrow().as_ref().unwrap())) as usize].clone() };
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
    }

    pub fn at(&self, i: Rc<RefCell<Option<i64>>>) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some({ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[((*i.borrow().as_ref().unwrap())) as usize].clone() })))
    }
}

fn main() {
    let mut names = Rc::new(RefCell::new(Some(Names(Rc::new(RefCell::new(Some(vec!["ada".to_string(), "grace".to_string(), "lin".to_string()])))))));
    (*names.borrow().as_ref().unwrap()).swap(Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(2))));
    println!("{} {}", format!("{}", (*(*names.borrow().as_ref().unwrap()).at(Rc::new(RefCell::new(Some(0)))).borrow().as_ref().unwrap())), format!("{}", (*(*names.borrow().as_ref().unwrap()).at(Rc::new(RefCell::new(Some(2)))).borrow().as_ref().unwrap())));
}