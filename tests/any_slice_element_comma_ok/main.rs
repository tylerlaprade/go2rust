use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
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
pub struct T {
    pub n: Rc<RefCell<Option<i64>>>,
}

impl T {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            n: __go_clone_0_0,
        }
    }
}


impl Default for T {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            n: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for T {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.n.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


fn main() {
    let mut items = Rc::new(RefCell::new(Some(vec![Box::new(Rc::new(RefCell::new(Some(vec![1i64, 2i64]))).clone()) as Box<dyn Any>, Box::new(Rc::new(RefCell::new(Some(T { n: Rc::new(RefCell::new(Some(3))), ..Default::default() }))).clone()) as Box<dyn Any>, Box::new(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(1i64))))])))).clone()) as Box<dyn Any>])));
    let (mut v, mut ok) = ({
        let __idx = (0) as usize;
        let __seq_holder = items.clone();
        let __seq_guard = __seq_holder.borrow();
        match __seq_guard.as_ref().expect("nil []any in type assertion")[__idx].downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>() {
            Some(typed_val) => (typed_val.clone(), true),
            None => (Rc::new(RefCell::new(None::<Vec<i64>>)), false),
        }
    });
    println!("{} {}", format!("{}", format_slice(&v)), format!("{}", ok));
    let (mut p, mut ok) = ({
        let __idx = (1) as usize;
        let __seq_holder = items.clone();
        let __seq_guard = __seq_holder.borrow();
        match __seq_guard.as_ref().expect("nil []any in type assertion")[__idx].downcast_ref::<Rc<RefCell<Option<T>>>>() {
            Some(typed_val) => (typed_val.clone(), true),
            None => (Rc::new(RefCell::new(None::<T>)), false),
        }
    });
    println!("{} {}", format!("{}", (*(*p.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())), format!("{}", ok));
    let (mut m, mut ok) = ({
        let __idx = (2) as usize;
        let __seq_holder = items.clone();
        let __seq_guard = __seq_holder.borrow();
        match __seq_guard.as_ref().expect("nil []any in type assertion")[__idx].downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>() {
            Some(typed_val) => (typed_val.clone(), true),
            None => (Rc::new(RefCell::new(None::<BTreeMap<String, Rc<RefCell<Option<i64>>>>>)), false),
        }
    });
    println!("{} {}", format!("{}", { let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", ok));
    { let (__tmp_0, __tmp_1) = ({
        let __idx = (0) as usize;
        let __seq_holder = items.clone();
        let __seq_guard = __seq_holder.borrow();
        match __seq_guard.as_ref().expect("nil []any in type assertion")[__idx].downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>() {
            Some(typed_val) => (typed_val.clone(), true),
            None => (Rc::new(RefCell::new(None::<BTreeMap<String, Rc<RefCell<Option<i64>>>>>)), false),
        }
    }); ok = __tmp_1; };
    println!("{}", format!("{}", ok));
    let (mut q, mut ok) = ({
        let __idx = (0) as usize;
        let __seq_holder = items.clone();
        let __seq_guard = __seq_holder.borrow();
        match __seq_guard.as_ref().expect("nil []any in type assertion")[__idx].downcast_ref::<Rc<RefCell<Option<T>>>>() {
            Some(typed_val) => (typed_val.clone(), true),
            None => (Rc::new(RefCell::new(None::<T>)), false),
        }
    });
    println!("{} {}", format!("{}", { let __nil_result = (*q.borrow()).is_none(); __nil_result }), format!("{}", ok));
}