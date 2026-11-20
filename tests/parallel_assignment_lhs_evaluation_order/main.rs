use std::cell::{RefCell};
use std::fmt::{Display};
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

pub(crate) static trace: GoGlobal<String> = GoGlobal::new();


fn __go_init_globals() {
    *trace.borrow_mut() = Some(String::new());
}


pub fn target(label: Rc<RefCell<Option<String>>>, values: Rc<RefCell<Option<Vec<i32>>>>) -> Rc<RefCell<Option<Vec<i32>>>> {
    { (*trace.borrow_mut().as_mut().unwrap()).push_str(&format!("{}{}", (*label.borrow().as_ref().unwrap()), "|".to_string())); };
    values.clone()
}

pub fn index(label: Rc<RefCell<Option<String>>>, value: Rc<RefCell<Option<i32>>>) -> i32 {
    { (*trace.borrow_mut().as_mut().unwrap()).push_str(&format!("{}{}", (*label.borrow().as_ref().unwrap()), "|".to_string())); };
    (*value.borrow().as_ref().unwrap())
}

pub fn result(label: Rc<RefCell<Option<String>>>, value: Rc<RefCell<Option<i32>>>) -> i32 {
    { (*trace.borrow_mut().as_mut().unwrap()).push_str(&format!("{}{}", (*label.borrow().as_ref().unwrap()), "|".to_string())); };
    (*value.borrow().as_ref().unwrap())
}

fn main() {
    __go_init_all();
    let mut left = Rc::new(RefCell::new(Some(vec![0, 0])));
    let mut right = Rc::new(RefCell::new(Some(vec![0, 0])));
    let mut __go_lhs_sequence_0 = Rc::new(RefCell::new(Some(vec![10])));
    let mut __go_lhs_index_0 = Rc::new(RefCell::new(Some(0)));
    {
        let __go_lhs_sequence_0_ = target(Rc::new(RefCell::new(Some("left sequence".to_string()))), left.clone()).clone();
        let __go_lhs_index_0_ = (index(Rc::new(RefCell::new(Some("left index".to_string()))), Rc::new(RefCell::new(Some(0))))) as usize;
        let __go_lhs_sequence_1 = target(Rc::new(RefCell::new(Some("right sequence".to_string()))), right.clone()).clone();
        let __go_lhs_index_1 = (index(Rc::new(RefCell::new(Some("right index".to_string()))), Rc::new(RefCell::new(Some(1))))) as usize;
        let __tmp_0 = result(Rc::new(RefCell::new(Some("left result".to_string()))), Rc::new(RefCell::new(Some((*__go_lhs_sequence_0.borrow().as_ref().unwrap())[((*__go_lhs_index_0.borrow().as_ref().unwrap())) as usize].clone()))));
        let __tmp_1 = result(Rc::new(RefCell::new(Some("right result".to_string()))), Rc::new(RefCell::new(Some(20))));
        (*__go_lhs_sequence_0_.borrow_mut().as_mut().unwrap())[__go_lhs_index_0_] = __tmp_0;
        (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
    };
    println!("{}", format!("{}", { let __v = (*trace.borrow().as_ref().unwrap()).clone(); __v }));
    println!("{} {}", format!("{}", format_slice(&left)), format!("{}", format_slice(&right)));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
