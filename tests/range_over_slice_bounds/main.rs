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

#[derive(Debug, Clone)]
pub struct scanner {
    pub offset: Rc<RefCell<Option<i32>>>,
    pub data: Rc<RefCell<Option<Vec<u8>>>>,
}

impl scanner {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.offset.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = self.data.clone();
        Self {
            offset: __go_clone_0_0,
            data: __go_clone_1_0,
        }
    }
}


impl Default for scanner {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        Self {
            offset: __go_default_0_0,
            data: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for scanner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.offset.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.data));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl scanner {
    pub fn advance(&mut self) {
        for (i, b) in { let __seq_holder = self.data.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (*self.offset.clone().borrow().as_ref().unwrap()) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }.iter().copied().enumerate() {
        if b == (' ' as u8) {
        { let __target = self.offset.clone(); let __rhs = (i as i32); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        break
    }
    }
    }
}

pub fn index_sum(n: Rc<RefCell<Option<i32>>>) -> i32 {
    let mut a: Rc<RefCell<Option<[i32; 4]>>> = Rc::new(RefCell::new(Some(std::array::from_fn(|_| 0))));
    let mut sum = Rc::new(RefCell::new(Some(0)));
    for i in 0..({ let __seq_holder = a.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = ((*n.borrow().as_ref().unwrap())) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }.len()) {
        { let __rhs = (i as i32); let mut guard = sum.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    return (*sum.borrow().as_ref().unwrap());
}

fn main() {
    println!("{} {} {}", format!("{}", index_sum(Rc::new(RefCell::new(Some(0))))), format!("{}", index_sum(Rc::new(RefCell::new(Some(3))))), format!("{}", index_sum(Rc::new(RefCell::new(Some(4))))));
    let mut s = Rc::new(RefCell::new(Some(scanner { offset: Rc::new(RefCell::new(Some(2))), data: Rc::new(RefCell::new(Some((" aBC d".to_string()).as_bytes().to_vec()))), ..Default::default() })));
    (*s.borrow_mut().as_mut().unwrap()).advance();
    println!("{}", format!("{}", (*(*s.borrow().as_ref().unwrap()).offset.borrow().as_ref().unwrap())));
}