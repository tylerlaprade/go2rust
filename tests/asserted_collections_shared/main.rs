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

#[derive(Debug, Clone, Default)]
pub struct Numbers(pub Rc<RefCell<Option<Vec<i32>>>>);

impl Display for Numbers {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


fn main() {
    let mut counts: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(1))))])))).clone()) as Box<dyn Any>)));
    let mut m = ({ let val = counts.clone(); let guard = val.borrow(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } });
    { let __map_key = "b".to_string(); let __map_value = Rc::new(RefCell::new(Some(2))); (*m.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __map_holder = m.clone(); let __key = "a".to_string(); let mut __map_guard = __map_holder.borrow_mut(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Rc::new(RefCell::new(Some(0)))); let mut __value = __entry.borrow_mut(); * __value = Some((*__value.as_ref().unwrap()).clone() + 1); }
    println!("{} {}", format!("{}", (*({ let val = counts.clone(); let guard = val.borrow(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } }).borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", { let __map_holder = ({ let val = counts.clone(); let guard = val.borrow(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } }).clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));

    let mut c = ({ let val = counts.clone(); let guard = val.borrow(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } });
    { let __map_key = "x".to_string(); let __map_value = Rc::new(RefCell::new(Some(7))); (*c.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{}", format!("{}", { let __map_holder = ({ let val = counts.clone(); let guard = val.borrow(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } }).clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"x".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));

    let counts_closure_clone = counts.clone(); { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        let mut inner = ({ let val = counts_closure_clone.clone(); let guard = val.borrow(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } });
        { let __map_key = "y".to_string(); let __map_value = Rc::new(RefCell::new(Some(5))); (*inner.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    println!("{}", format!("{}", (*({ let val = counts.clone(); let guard = val.borrow(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } }).borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));

    let mut list: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(vec![1, 2, 3]))).clone()) as Box<dyn Any>)));
    let mut items = ({ let val = list.clone(); let guard = val.borrow(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Rc<RefCell<Option<Vec<i32>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } });
    { let __idx = (0) as usize; let __rhs = 10; (*items.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {}", format!("{}", (*({ let val = list.clone(); let guard = val.borrow(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Rc<RefCell<Option<Vec<i32>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } }).borrow().as_ref().unwrap())[(0) as usize].clone()), format!("{}", (*items.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));

        // A var of another slice type is a value of that type made from the
        // asserted slice.
    let mut typed: Rc<RefCell<Option<Numbers>>> = Rc::new(RefCell::new(Some(Numbers(Rc::new(RefCell::new(Some(({
        let val = list.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            { let __handle = any_val.downcast_ref::<Rc<RefCell<Option<Vec<i32>>>>>().expect("type assertion failed").clone(); let __value = (*__handle.borrow()).clone().unwrap_or_default(); __value }
        } else {
            panic!("type assertion on nil interface")
        }
    }))))))));
    let mut plain: Rc<RefCell<Option<Vec<i32>>>> = ({ let val = list.clone(); let guard = val.borrow(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Rc<RefCell<Option<Vec<i32>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } });
    { let __idx = (1) as usize; let __rhs = 20; (*plain.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {}", format!("{}", { let __slice_holder = { let __named_slice = (*typed.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }), format!("{}", (*({ let val = list.clone(); let guard = val.borrow(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Rc<RefCell<Option<Vec<i32>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } }).borrow().as_ref().unwrap())[(1) as usize].clone()));

    let mut names: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(vec!["x".to_string()]))).clone()) as Box<dyn Any>)));
    let mut ns = ({ let val = names.clone(); let guard = val.borrow(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Rc<RefCell<Option<Vec<String>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } });
    { let __idx = (0) as usize; let __rhs = "z".to_string(); (*ns.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    println!("{}", format!("{}", format_slice(&({ let val = names.clone(); let guard = val.borrow(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Rc<RefCell<Option<Vec<String>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } }))));
}