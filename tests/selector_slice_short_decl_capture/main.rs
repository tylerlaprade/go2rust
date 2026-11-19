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

impl dict {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.tparams.clone();
        Self {
            tparams: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for dict {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_slice(&self.tparams));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Clone, Default)]
pub struct reader {
    pub dict: Rc<RefCell<Option<dict>>>,
    pub later: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>>>>>>,
}

impl reader {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.dict.clone();
        let __go_clone_1_0 = self.later.clone();
        Self {
            dict: __go_clone_0_0,
            later: __go_clone_1_0,
        }
    }
}

impl std::fmt::Display for reader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.dict.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", { let __guard = self.later.borrow(); match __guard.as_ref() { Some(__v) => format!("[{}]", std::iter::repeat("<func>").take(__v.len()).collect::<Vec<_>>().join(" ")), None => "[]".to_string() } });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl reader {
    pub fn collect(&mut self, vals: Rc<RefCell<Option<Vec<i32>>>>) {
        let mut tparams = (*self.dict.borrow().as_ref().unwrap()).tparams.clone();
        let tparams_closure_clone = tparams.clone(); let vals_closure_clone = vals.clone(); { let new_val = { let __append_target = self.later.clone(); let __append_value = Rc::new(RefCell::new(Some(Box::new(move || {
        { let __range_holder = vals_closure_clone.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, val) in __range_values.iter().copied().enumerate() {
        { let __idx = (i) as usize; let __rhs = val; (*tparams_closure_clone.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    } }
    }) as Box<dyn FnMut() -> ()>))); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; self.later = new_val; };
    }
}

fn main() {
    let mut r = Rc::new(RefCell::new(Some(reader { dict: Rc::new(RefCell::new(Some(dict { tparams: Rc::new(RefCell::new(Some(vec![1, 2]))), ..Default::default() }))).clone(), later: Rc::new(RefCell::new(Some(Vec::<Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>>>::new()))), ..Default::default() })));
    let mut vals = Rc::new(RefCell::new(Some(vec![3, 4])));
    (*r.borrow_mut().as_mut().unwrap()).collect(vals.clone());
    { let __f_holder = (*(*r.borrow().as_ref().unwrap()).later.borrow().as_ref().unwrap())[(0) as usize].clone(); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    println!("{} {}", format!("{}", (*(*(*r.borrow().as_ref().unwrap()).dict.borrow().as_ref().unwrap()).tparams.borrow().as_ref().unwrap())[(0) as usize].clone()), format!("{}", (*(*(*r.borrow().as_ref().unwrap()).dict.borrow().as_ref().unwrap()).tparams.borrow().as_ref().unwrap())[(1) as usize].clone()));
}