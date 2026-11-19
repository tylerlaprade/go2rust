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

#[derive(Debug, Clone, Default)]
pub struct Numbers(pub Rc<RefCell<Option<Vec<i32>>>>);

impl Display for Numbers {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


fn main() {
    let mut m = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Numbers>>>>::from([]))));
    let mut one = { let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"zzz".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Rc::new(RefCell::new(Some(Default::default())))) };
    let (mut two, mut ok) = { let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); match __map_guard.as_ref().and_then(|__map| __map.get(&"zzz".to_string())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Rc::new(RefCell::new(Some(Default::default()))), false) } };
    println!("{} {} {} {} {}", format!("{}", { let __slice_holder = { let __named_slice = (*one.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }), format!("{}", { let __slice_holder = { let __named_slice = one.borrow().as_ref().map(|__v| __v.0.clone()).unwrap_or_else(|| Rc::new(RefCell::new(None))); __named_slice }; let __slice_guard = __slice_holder.borrow(); (*__slice_guard).is_none() }), format!("{}", ok), format!("{}", { let __slice_holder = { let __named_slice = (*two.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }), format!("{}", { let __slice_holder = { let __named_slice = two.borrow().as_ref().map(|__v| __v.0.clone()).unwrap_or_else(|| Rc::new(RefCell::new(None))); __named_slice }; let __slice_guard = __slice_holder.borrow(); (*__slice_guard).is_none() }));

    let mut s = Rc::new(RefCell::new(Some(Numbers(Rc::new(RefCell::new(Some(vec![1])))))));
    println!("{} {}", format!("{}", { let __slice_holder = { let __named_slice = s.borrow().as_ref().map(|__v| __v.0.clone()).unwrap_or_else(|| Rc::new(RefCell::new(None))); __named_slice }; let __slice_guard = __slice_holder.borrow(); (*__slice_guard).is_none() }), format!("{}", { let __slice_holder = { let __named_slice = s.borrow().as_ref().map(|__v| __v.0.clone()).unwrap_or_else(|| Rc::new(RefCell::new(None))); __named_slice }; let __slice_guard = __slice_holder.borrow(); (*__slice_guard).is_some() }));
    *s.borrow_mut() = None;
    println!("{} {}", format!("{}", { let __slice_holder = { let __named_slice = s.borrow().as_ref().map(|__v| __v.0.clone()).unwrap_or_else(|| Rc::new(RefCell::new(None))); __named_slice }; let __slice_guard = __slice_holder.borrow(); (*__slice_guard).is_none() }), format!("{}", { let __slice_holder = { let __named_slice = s.borrow().as_ref().map(|__v| __v.0.clone()).unwrap_or_else(|| Rc::new(RefCell::new(None))); __named_slice }; let __slice_guard = __slice_holder.borrow(); (*__slice_guard).is_some() }));
}