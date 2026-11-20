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
pub struct Matrix(pub Rc<RefCell<Option<Vec<Vec<f64>>>>>);


#[derive(Debug, Clone, Default)]
pub struct Ints(pub Rc<RefCell<Option<Vec<i64>>>>);

impl Display for Ints {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


#[derive(Debug, Clone, Default)]
pub struct Words(pub Rc<RefCell<Option<Vec<String>>>>);

impl Display for Words {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


impl Matrix {
    pub fn transpose(&self) -> Rc<RefCell<Option<Matrix>>> {
        let mut t = Rc::new(RefCell::new(Some(Matrix(Rc::new(RefCell::new(Some(vec![vec![]; ({ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() }.len()) as usize])))))));
        for i in 0..(({ let __range_holder = { let __named_slice = (*t.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __range_guard = __range_holder.borrow(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        { let __idx = (i) as usize; let __rhs = (*Rc::new(RefCell::new(Some(vec![0.0; ({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as usize]))).borrow().as_ref().unwrap()).clone(); (*{ let __named_slice = (*t.borrow().as_ref().unwrap()).0.clone(); __named_slice }.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
        for j in 0..(({ let __range_holder = self.0.clone(); let __range_guard = __range_holder.borrow(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        { let __seq = { let __named_slice = (*t.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __outer_idx = (i) as usize; let __idx = (j) as usize; let __rhs = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(j) as usize].clone() }[(i) as usize].clone(); (*__seq.borrow_mut().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
    }
    }
        return t.clone();
    }
}

pub fn mk() -> Rc<RefCell<Option<Ints>>> {
    Rc::new(RefCell::new(Some(Ints(Rc::new(RefCell::new(Some(vec![1i64, 2i64])))))))
}

fn main() {
    let mut m = Rc::new(RefCell::new(Some(Matrix(Rc::new(RefCell::new(Some(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]])))))));
    let mut xs = Rc::new(RefCell::new(Some(Ints(Rc::new(RefCell::new(Some(vec![5i64, 6i64])))))));
    let mut ws = Rc::new(RefCell::new(Some(Words(Rc::new(RefCell::new(Some(vec!["a".to_string(), "b".to_string()])))))));
    println!("{} {} {}", format!("{}", format_nested_slice(&{ let __named_slice = (*m.borrow().as_ref().unwrap()).0.clone(); __named_slice })), format!("{}", format_slice(&{ let __named_slice = (*xs.borrow().as_ref().unwrap()).0.clone(); __named_slice })), format!("{}", format_slice(&{ let __named_slice = (*ws.borrow().as_ref().unwrap()).0.clone(); __named_slice })));
    println!("{} {}", format!("{}", format_nested_slice(&{ let __named_slice = (*(*m.borrow().as_ref().unwrap()).transpose().borrow().as_ref().unwrap()).0.clone(); __named_slice })), format!("{}", format_slice(&{ let __named_slice = (*mk().borrow().as_ref().unwrap()).0.clone(); __named_slice })));
    print!("{} {}\n", format_nested_slice(&{ let __named_slice = (*m.borrow().as_ref().unwrap()).0.clone(); __named_slice }), format_slice(&{ let __named_slice = (*mk().borrow().as_ref().unwrap()).0.clone(); __named_slice }));
    { let __seq = { let __named_slice = (*m.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __outer_idx = (1) as usize; let __idx = (2) as usize; let __rhs = 9.0; (*__seq.borrow_mut().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
    let mut grid = Rc::new(RefCell::new(Some(Matrix(Rc::new(RefCell::new(Some(vec![vec![]; (2) as usize])))))));
    for i in 0..(({ let __range_holder = { let __named_slice = (*grid.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __range_guard = __range_holder.borrow(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        { let __idx = (i) as usize; let __rhs = (*Rc::new(RefCell::new(Some(vec![0.0; (2) as usize]))).borrow().as_ref().unwrap()).clone(); (*{ let __named_slice = (*grid.borrow().as_ref().unwrap()).0.clone(); __named_slice }.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
        { let __seq = { let __named_slice = (*grid.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __outer_idx = (i) as usize; let __idx = (i) as usize; let __rhs = 1.0; (*__seq.borrow_mut().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
    }
    println!("{} {} {}", format!("{}", format_nested_slice(&{ let __named_slice = (*m.borrow().as_ref().unwrap()).0.clone(); __named_slice })), format!("{}", format_nested_slice(&{ let __named_slice = (*grid.borrow().as_ref().unwrap()).0.clone(); __named_slice })), format!("{}", { let __slice_holder = { let __named_slice = (*(*m.borrow().as_ref().unwrap()).transpose().borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }));
}