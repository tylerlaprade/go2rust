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
pub struct dict {
    pub tparams: Rc<RefCell<Option<Vec<i32>>>>,
}

impl std::fmt::Display for dict {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice(&self.tparams))
    }
}


#[derive(Clone, Default)]
pub struct reader {
    pub dict: Rc<RefCell<Option<dict>>>,
    pub later: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Box<dyn Fn() -> ()>>>>>>>>,
}

impl std::fmt::Display for reader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.dict.borrow().as_ref().unwrap()), { let __guard = self.later.borrow(); match __guard.as_ref() { Some(__v) => format!("[{}]", std::iter::repeat("<func>").take(__v.len()).collect::<Vec<_>>().join(" ")), None => "[]".to_string() } })
    }
}


impl reader {
    pub fn collect(&mut self, vals: Rc<RefCell<Option<Vec<i32>>>>) {
        let mut tparams = (*self.dict.borrow().as_ref().unwrap()).tparams.clone();
        let tparams_closure_clone = tparams.clone(); let vals_closure_clone = vals.clone(); {(*self.later.borrow_mut()).get_or_insert_with(Vec::new).push(Rc::new(RefCell::new(Some(Box::new(move || {
        { let __range_holder = vals.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for (i, val) in __range_values.iter().copied().enumerate() {
        (*tparams_closure_clone.borrow_mut().as_mut().unwrap())[(i) as usize] = val;
    } }
    }) as Box<dyn Fn() -> ()>)))); self.later.clone()};
    }
}

fn main() {
    let mut r = Rc::new(RefCell::new(Some(reader { dict: Rc::new(RefCell::new(Some(dict { tparams: Rc::new(RefCell::new(Some(vec![1, 2]))), ..Default::default() }))).clone(), later: Rc::new(RefCell::new(Some(Vec::<Rc<RefCell<Option<Box<dyn Fn() -> ()>>>>>::new()))), ..Default::default() })));
    let mut vals = Rc::new(RefCell::new(Some(vec![3, 4])));
    (*r.borrow_mut().as_mut().unwrap()).collect(vals.clone());
    { let __f_holder = (*(*r.borrow().as_ref().unwrap()).later.borrow().as_ref().unwrap())[(0) as usize].clone(); let __f_guard = __f_holder.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)() };
    println!("{} {}", (*(*(*r.borrow().as_ref().unwrap()).dict.borrow().as_ref().unwrap()).tparams.borrow().as_ref().unwrap())[(0) as usize].clone(), (*(*(*r.borrow().as_ref().unwrap()).dict.borrow().as_ref().unwrap()).tparams.borrow().as_ref().unwrap())[(1) as usize].clone());
}