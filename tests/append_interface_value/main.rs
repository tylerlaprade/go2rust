use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};


fn format_any(value: &dyn Any) -> String {
    if let Some(v) = value.downcast_ref::<i32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<String>() {
        v.clone()
    } else if let Some(v) = value.downcast_ref::<&str>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<bool>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<pair>() {
        format!("{}", v)
    } else {
        "<unknown>".to_string()
    }
}

fn format_any_slice_values(slice: &Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> String {
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| format_any(v.as_ref())).collect();
        formatted.join(" ")
    } else {
        String::new()
    }
}

fn format_any_slice(slice: &Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> String {
    format!("[{}]", format_any_slice_values(slice))
}

fn format_any_variadic(slice: &Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> String {
    format_any_slice_values(slice)
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
    if let Some(v) = value.downcast_ref::<pair>() { return Box::new(v.clone()) as Box<dyn Any>; }

    panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>")
}

#[derive(Debug, Clone)]
pub struct pair {
    pub k: Rc<RefCell<Option<String>>>,
}

impl pair {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.k.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            k: __go_clone_0_0,
        }
    }
}


impl Default for pair {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            k: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for pair {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.k.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


fn main() {
    let mut kept: Rc<RefCell<Option<Vec<Box<dyn Any>>>>> = Rc::new(RefCell::new(None));
    let mut x: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new("s".to_string()) as Box<dyn Any>)));
    { let new_val = { let __append_target = kept.clone(); let __append_value = { let __any_holder = x.clone(); let __any_guard = __any_holder.borrow(); go_any_clone(__any_guard.as_ref().expect("nil interface in variadic any argument").as_ref()) }; (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; kept = new_val; };
    let mut y: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(3) as Box<dyn Any>)));
    { let new_val = { let __append_target = kept.clone(); let __append_values = vec![{ let __any_holder = y.clone(); let __any_guard = __any_holder.borrow(); go_any_clone(__any_guard.as_ref().expect("nil interface in variadic any argument").as_ref()) }, { let __any_holder = x.clone(); let __any_guard = __any_holder.borrow(); go_any_clone(__any_guard.as_ref().expect("nil interface in variadic any argument").as_ref()) }]; (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).extend(__append_values); __append_target.clone() }; kept = new_val; };
    let mut p: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(pair { k: Rc::new(RefCell::new(Some("k".to_string()))), ..Default::default() }) as Box<dyn Any>)));
    { let new_val = { let __append_target = kept.clone(); let __append_value = { let __any_holder = p.clone(); let __any_guard = __any_holder.borrow(); go_any_clone(__any_guard.as_ref().expect("nil interface in variadic any argument").as_ref()) }; (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; kept = new_val; };
    { let new_val = Box::new(4) as Box<dyn Any>; *y.borrow_mut() = Some(new_val); };
    println!("{}", format_any_variadic(&kept));
    println!("{} {}", format!("{}", (*kept.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", format_any(y.borrow().as_ref().unwrap().as_ref())));
    let mut m = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Box<dyn Any>>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(Box::new(1) as Box<dyn Any>))))]))));
    { let new_val = { let __append_target = kept.clone(); let __append_value = { let __any_holder = { let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __any_guard = __any_holder.borrow(); go_any_clone(__any_guard.as_ref().expect("nil interface in variadic any argument").as_ref()) }; (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; kept = new_val; };
    println!("{}", format!("{}", (*kept.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
}