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
    pub rows: Rc<RefCell<Option<Vec<Vec<i64>>>>>,
}

impl board {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.rows.clone();
        Self {
            rows: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_nested_slice(&self.rows));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub(crate) struct GoGlobal<T> {
    value: std::cell::UnsafeCell<Option<T>>,
}
unsafe impl<T> Sync for GoGlobal<T> {}
impl<T> GoGlobal<T> {
    pub(crate) const fn new() -> Self {
        Self { value: std::cell::UnsafeCell::new(None) }
    }
    pub(crate) fn borrow(&'static self) -> &'static Option<T> {
        unsafe { &*self.value.get() }
    }
    pub(crate) fn borrow_mut(&'static self) -> &'static mut Option<T> {
        unsafe { &mut *self.value.get() }
    }
    pub(crate) fn clone(&'static self) -> std::rc::Rc<std::cell::RefCell<Option<T>>> where T: Clone {
        std::rc::Rc::new(std::cell::RefCell::new(self.borrow().clone()))
    }
}

pub(crate) static global: GoGlobal<Vec<Vec<i64>>> = GoGlobal::new();


fn __go_init_globals() {
    *global.borrow_mut() = Some(vec![]);
}


fn main() {
    __go_init_all();
    let mut grid = Rc::new(RefCell::new(Some(vec![vec![1i64, 2i64], vec![3i64, 4i64], Vec::new()])));
    println!("{}", format!("{}", format_nested_slice(&grid)));
    println!("{}", format!("{}", format_nested_slice(&Rc::new(RefCell::new(Some(vec![vec!["a".to_string(), "b".to_string()], vec!["c".to_string()]]))))));

    let mut empty: Rc<RefCell<Option<Vec<Vec<i64>>>>> = Rc::new(RefCell::new(None));
    println!("{}", format!("{}", format_nested_slice(&empty)));

    let mut cube = Rc::new(RefCell::new(Some(vec![vec![vec![5i64], vec![6i64, 7i64]]])));
    println!("{} {}", format!("{}", { let __formatted = format_nested_slice_values(&(*cube.borrow().as_ref().unwrap())[(0) as usize].clone()); __formatted }), format!("{}", { let __formatted = format_nested_slice_values(&(*cube.borrow().as_ref().unwrap())[(0) as usize].clone()); __formatted }));
    { let __range_holder = cube.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for layer in __range_values.iter() {
        println!("{}", format!("{}", { let __formatted = format_nested_slice_values(&layer); __formatted }));
    } }
    println!("{}", format!("{}", { let __formatted = format_nested_slice_values(&(*board { rows: Rc::new(RefCell::new(Some(vec![vec![8i64]]))), ..Default::default() }.rows.borrow().as_ref().unwrap()).clone()); __formatted }));
    println!("{} {}", format!("{}", { let __rows = global.borrow(); match __rows.as_ref() { Some(rows) => format_nested_slice_values(rows.as_slice()), None => "[]".to_string() } }), format!("{}", { let __rows = global.borrow(); match __rows.as_ref() { Some(rows) => format_nested_slice_values(rows.as_slice()), None => "[]".to_string() } }));
    { let new_val = { let __collection_holder = Rc::new(RefCell::new(Some(vec![vec![9i64]]))).clone(); let __collection_guard = __collection_holder.borrow(); (*__collection_guard).clone() }; *global.borrow_mut() = new_val; };
    println!("{} {}", format!("{}", { let __rows = global.borrow(); match __rows.as_ref() { Some(rows) => format_nested_slice_values(rows.as_slice()), None => "[]".to_string() } }), format!("{}", { let __rows = global.borrow(); match __rows.as_ref() { Some(rows) => format_nested_slice_values(rows.as_slice()), None => "[]".to_string() } }));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
