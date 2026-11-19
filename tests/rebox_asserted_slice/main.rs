use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
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

fn go_any_clone(value: &dyn Any) -> Box<dyn Any> {
    if let Some(v) = value.downcast_ref::<i32>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<i64>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<i8>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<i16>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u32>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u64>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u8>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u16>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<usize>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<isize>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<f64>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<f32>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<String>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<&'static str>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<bool>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<char>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<i32>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }

    panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>")
}

fn main() {
    let mut a = Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(vec![1, 2]))).clone()) as Box<dyn Any>)));
    let mut b = Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(({
        let val = a.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            { let __handle = any_val.downcast_ref::<Rc<RefCell<Option<Vec<i32>>>>>().expect("type assertion failed").clone(); let __value = (*__handle.borrow()).clone().unwrap_or_default(); __value }
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone())))) as Box<dyn Any>)));
    let mut c = Rc::new(RefCell::new(Some(({
        let val = b.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            { let __handle = any_val.downcast_ref::<Rc<RefCell<Option<Vec<i32>>>>>().expect("type assertion failed").clone(); let __value = (*__handle.borrow()).clone().unwrap_or_default(); __value }
        } else {
            panic!("type assertion on nil interface")
        }
    }))));
    println!("{} {} {}", format!("{}", (*c.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*c.borrow().as_ref().unwrap())[(0) as usize].clone()), format!("{}", (*c.borrow().as_ref().unwrap())[(1) as usize].clone()));

    let mut holder: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(({
        let val = a.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            { let __handle = any_val.downcast_ref::<Rc<RefCell<Option<Vec<i32>>>>>().expect("type assertion failed").clone(); let __value = (*__handle.borrow()).clone().unwrap_or_default(); __value }
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone())))) as Box<dyn Any>)));
    let (mut d, mut ok) = ({
        let val = holder.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Vec<i32>>>>>() {
                (typed_val.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<Vec<i32>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Vec<i32>>)), false)
        }
    });
    println!("{} {}", format!("{}", ok), format!("{}", format_slice(&d)));

    let mut m = Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([("k".to_string(), Rc::new(RefCell::new(Some(1))))])))).clone()) as Box<dyn Any>)));
    let mut n = Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(({
        let val = m.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            { let __handle = any_val.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>>().expect("type assertion failed").clone(); let __value = (*__handle.borrow()).clone().unwrap_or_default(); __value }
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone())))) as Box<dyn Any>)));
    let (mut back, mut ok) = ({
        let val = n.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>>() {
                (typed_val.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<BTreeMap<String, Rc<RefCell<Option<i32>>>>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<BTreeMap<String, Rc<RefCell<Option<i32>>>>>)), false)
        }
    });
    println!("{} {} {}", format!("{}", ok), format!("{}", { let __map_holder = back.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"k".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", (*back.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));

    let mut items = Rc::new(RefCell::new(Some(vec![Box::new(Rc::new(RefCell::new(Some(({
        let val = a.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            { let __handle = any_val.downcast_ref::<Rc<RefCell<Option<Vec<i32>>>>>().expect("type assertion failed").clone(); let __value = (*__handle.borrow()).clone().unwrap_or_default(); __value }
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone())))) as Box<dyn Any>, Box::new(Rc::new(RefCell::new(Some(({
        let val = m.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            { let __handle = any_val.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>>().expect("type assertion failed").clone(); let __value = (*__handle.borrow()).clone().unwrap_or_default(); __value }
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone())))) as Box<dyn Any>])));
    { let __range_holder = items.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.iter().map(|__e| go_any_clone(__e.as_ref())).collect::<Vec<_>>()).unwrap_or_default(); drop(__range_guard); for item in __range_values.iter() {
        {
    let _ts_ref = item;
    let _ts_is_nil = false;
    let _ts_val: Option<&dyn Any> = Some(_ts_ref.as_ref() as &dyn Any);
    if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Vec<i32>>>>>()).is_some() {
        let v = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Vec<i32>>>>>()).unwrap().clone();
        println!("{} {}", format!("{}", "slice".to_string()), format!("{}", (*v.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>>()).is_some() {
        let v = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>>()).unwrap().clone();
        println!("{} {}", format!("{}", "map".to_string()), format!("{}", (*v.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));;
    }
    }
    } }
}