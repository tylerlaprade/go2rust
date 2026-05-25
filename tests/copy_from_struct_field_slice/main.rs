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
pub struct Group {
    pub items: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Item>>>>>>>,
}

impl Group {
    pub fn __go_value_clone(&self) -> Self {
        Self { items: self.items.clone() }
    }
}

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice_wrapped(&self.items))
    }
}


#[derive(Debug, Clone)]
pub struct Item {
    pub n: Rc<RefCell<Option<i32>>>,
}

impl Item {
    pub fn __go_value_clone(&self) -> Self {
        Self { n: { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Item {
    fn default() -> Self {
        Self { n: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.n.borrow().as_ref().unwrap()))
    }
}


/// Mirrors go/ast/filter.go:489's pattern:
///
///	i += copy(dst[i:], f.Field)
///
/// where the field is a slice of pointers on a wrapped struct value.
fn main() {
    let mut g = Rc::new(RefCell::new(Some(Group { items: Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Item { n: Rc::new(RefCell::new(Some(1))), ..Default::default() }))), Rc::new(RefCell::new(Some(Item { n: Rc::new(RefCell::new(Some(2))), ..Default::default() }))), Rc::new(RefCell::new(Some(Item { n: Rc::new(RefCell::new(Some(3))), ..Default::default() })))]))), ..Default::default() })));
    let mut h = Rc::new(RefCell::new(Some(Group { items: Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Item { n: Rc::new(RefCell::new(Some(4))), ..Default::default() }))), Rc::new(RefCell::new(Some(Item { n: Rc::new(RefCell::new(Some(5))), ..Default::default() })))]))), ..Default::default() })));
    let mut combined: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Item>>>>>>> = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(None)); (5) as usize])));
    let mut i = Rc::new(RefCell::new(Some(0)));
    { let __rhs = (*{ let _dst_start = ((*i.borrow().as_ref().unwrap())) as usize; let _dst_len = (*combined.borrow().as_ref().unwrap()).len() - _dst_start; let _src = (*(*g.borrow().as_ref().unwrap()).items.borrow().as_ref().unwrap()).clone(); let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*combined.borrow_mut().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Rc::new(RefCell::new(Some(_n as i32))) }.borrow().as_ref().unwrap()); let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    { let __rhs = (*{ let _dst_start = ((*i.borrow().as_ref().unwrap())) as usize; let _dst_len = (*combined.borrow().as_ref().unwrap()).len() - _dst_start; let _src = (*(*h.borrow().as_ref().unwrap()).items.borrow().as_ref().unwrap()).clone(); let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*combined.borrow_mut().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Rc::new(RefCell::new(Some(_n as i32))) }.borrow().as_ref().unwrap()); let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    { let __range_holder = combined.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for it in __range_values.iter() {
        println!("{}", format!("{}", (*(*it.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())));
    } }
}