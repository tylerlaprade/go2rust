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
pub struct Stack(pub Rc<RefCell<Option<Vec<i32>>>>);

impl Display for Stack {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


impl Stack {
    pub fn push(&mut self, n: Rc<RefCell<Option<i32>>>) {
        { let new_val = { let __base = self.0.clone(); let __base_guard = __base.borrow(); let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); drop(__base_guard); __values.push((*n.borrow().as_ref().unwrap()).clone()); Rc::new(RefCell::new(Some(Stack(Rc::new(RefCell::new(Some(__values))))))) }; *self = new_val.borrow_mut().take().unwrap_or_default(); };
    }

    pub fn pop(&mut self) -> i32 {
        let mut i = Rc::new(RefCell::new(Some({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
        if (*i.borrow().as_ref().unwrap()) == 0 {
        return -1;
    }
        let mut top = Rc::new(RefCell::new(Some({ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[((*i.borrow().as_ref().unwrap()) - 1) as usize].clone() })));
        { let new_val = Stack(Rc::new(RefCell::new(Some({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.borrow(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[(0) as usize..((*i.borrow().as_ref().unwrap()) - 1) as usize].to_vec() })))); *self = new_val; };
        (*top.borrow().as_ref().unwrap())
    }
}

fn main() {
    let mut s: Rc<RefCell<Option<Stack>>> = Rc::new(RefCell::new(Some(Default::default())));
    (*s.borrow_mut().as_mut().unwrap()).push(Rc::new(RefCell::new(Some(1))));
    (*s.borrow_mut().as_mut().unwrap()).push(Rc::new(RefCell::new(Some(2))));
    (*s.borrow_mut().as_mut().unwrap()).push(Rc::new(RefCell::new(Some(3))));
    println!("{}", format!("{}", (*s.borrow_mut().as_mut().unwrap()).pop()));
    println!("{}", format!("{}", (*s.borrow_mut().as_mut().unwrap()).pop()));
    println!("{}", format!("{}", (*s.borrow_mut().as_mut().unwrap()).pop()));
}