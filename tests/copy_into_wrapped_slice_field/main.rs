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

/// GAP: copy(dst, src) where dst is a wrapped slice held in a struct field.
/// transpileCopy's non-SliceExpr branch computes the length via
/// (TranspileExpression(dst)).len() without unwrapping the handle (E0599),
/// and only fully-unwraps the per-element place for *ast.Ident, not a
/// selector (E0608). Root cause: go/stdlib.go transpileCopy.
#[derive(Debug, Clone, Default)]
pub struct buf {
    pub dst: Rc<RefCell<Option<Vec<u8>>>>,
}

impl buf {
    pub fn __go_value_clone(&self) -> Self {
        Self { dst: self.dst.clone() }
    }
}

impl std::fmt::Display for buf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice(&self.dst))
    }
}


impl buf {
    pub fn fill(&mut self, src: Rc<RefCell<Option<Vec<u8>>>>) {
        { let new_val = Rc::new(RefCell::new(Some(vec![0; ((*src.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize]))); self.dst = new_val; };
        { let _src = { let __copy_src_holder = src.clone(); let __copy_src_guard = __copy_src_holder.borrow(); __copy_src_guard.as_ref().cloned().unwrap_or_default() }; let _n = std::cmp::min((*self.dst.borrow().as_ref().unwrap()).len(), _src.len()); for _i in 0.._n { (*self.dst.borrow_mut().as_mut().unwrap())[_i] = _src[_i].clone(); } Rc::new(RefCell::new(Some(_n as i32))) };
    }
}

fn main() {
    let mut b = Rc::new(RefCell::new(Some(buf { dst: Default::default() })));
    (*b.borrow_mut().as_mut().unwrap()).fill(Rc::new(RefCell::new(Some(("hello".to_string()).as_bytes().to_vec()))));
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8({ let __slice_holder = (*b.borrow().as_ref().unwrap()).dst.clone(); let __slice_guard = __slice_holder.borrow(); (*__slice_guard.as_ref().unwrap()).clone() }).unwrap()))).borrow().as_ref().unwrap())));
}