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
fn format_nested_slice<T, C, Inner>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Inner]>,
    Inner: AsRef<[T]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        format_nested_slice_values(s.as_ref())
    } else {
        "[]".to_string()
    }
}
fn format_nested_slice_values<T, Inner>(slice: &[Inner]) -> String
where
    Inner: AsRef<[T]>,
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|inner| format_slice_values(inner.as_ref())).collect();
    format!("[{}]", formatted.join(" "))
}

#[derive(Debug, Clone, Default)]
pub struct board {
    pub cells: Rc<RefCell<Option<Vec<Vec<i64>>>>>,
}

impl board {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.cells.clone();
        Self {
            cells: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_nested_slice(&self.cells));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


fn main() {
    let mut grid = Rc::new(RefCell::new(Some([std::array::from_fn::<_, 3, _>(|_| 0), std::array::from_fn::<_, 3, _>(|_| 0), std::array::from_fn::<_, 3, _>(|_| 0)])));
    for i in 0..(({ let __range_holder = grid.clone(); let __range_guard = __range_holder.borrow(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        for j in 0..({ let __range_len = (*grid.borrow().as_ref().unwrap())[(i) as usize].clone().len(); __range_len }) {
        { let __outer_idx = (i) as usize; let __idx = (j) as usize; let __rhs = i as i64 * j as i64; (*grid.borrow_mut().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
    }
    }
    println!("{}", format!("{}", format_nested_slice(&grid)));

    let mut rows = Rc::new(RefCell::new(Some(vec![vec!["a".to_string(), "b".to_string()], vec!["c".to_string()]])));
    for i in 0..(({ let __range_holder = rows.clone(); let __range_guard = __range_holder.borrow(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        for j in 0..({ let __range_len = (*rows.borrow().as_ref().unwrap())[(i) as usize].clone().len(); __range_len }) {
        { let __outer_idx = (i) as usize; let __idx = (j) as usize; let __rhs = "!".to_string(); let mut __seq_guard = rows.borrow_mut(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx]; __seq[__idx] = __seq[__idx].clone() + &__rhs; };
    }
        { let __append_idx = (i) as usize; let __append_value = "+".to_string(); (*rows.borrow_mut().as_mut().unwrap())[__append_idx].push(__append_value); (*rows.borrow().as_ref().unwrap())[__append_idx].clone() };
    }
    println!("{}", format!("{}", format_nested_slice(&rows)));

    let mut b = Rc::new(RefCell::new(Some(board { cells: Rc::new(RefCell::new(Some(vec![vec![1i64, 2i64], vec![3i64, 4i64]]))), ..Default::default() })));
    for r in 0..(({ let __range_holder = (*b.borrow().as_ref().unwrap()).cells.clone(); let __range_guard = __range_holder.borrow(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        for c in 0..({ let __range_len = (*(*b.borrow().as_ref().unwrap()).cells.borrow().as_ref().unwrap())[(r) as usize].clone().len(); __range_len }) {
        { let __outer_idx = (r) as usize; let __idx = (c) as usize; let __rhs = 10; let __seq_holder = (*b.borrow().as_ref().unwrap()).cells.clone(); let mut __seq_guard = __seq_holder.borrow_mut(); let __seq = &mut __seq_guard.as_mut().unwrap()[__outer_idx]; __seq[__idx] = __seq[__idx] * __rhs; };
    }
    }
    println!("{}", format!("{}", { let __formatted = format_nested_slice_values(&(*(*b.borrow().as_ref().unwrap()).cells.borrow().as_ref().unwrap()).clone()); __formatted }));

    let mut n = Rc::new(RefCell::new(Some(3i64)));
    let mut total = Rc::new(RefCell::new(Some(0i64)));
    for i in 0..({ let __range_len = (*n.borrow().as_ref().unwrap()); __range_len }) {
        { let new_val = 1; *n.borrow_mut() = Some(new_val); };
        { let __rhs = i; let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    println!("{} {}", format!("{}", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*total.borrow().as_ref().unwrap()).clone(); __v }));
}