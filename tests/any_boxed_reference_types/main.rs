use std::any::Any;
use std::cell::{RefCell};
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};


// Go expands a pointer to a struct only when the pointer is the value being
// printed. Inside a struct or a slice it prints the address. That is also
// what keeps a value that points back to itself from printing forever.
fn go_format_any_pointer(nested: bool, address: *const (), is_nil: impl FnOnce() -> bool, expand: impl FnOnce() -> String) -> String {
    if !nested {
        return expand();
    }
    if is_nil() { "<nil>".to_string() } else { format!("{:p}", address) }
}
fn format_map<K: Display + Ord + Clone, V>(map: &Rc<RefCell<Option<BTreeMap<K, Rc<RefCell<Option<V>>>>>>>) -> String
where
    V: Display,
{
    let guard = map.borrow();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());

        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.borrow();
                if let Some(ref val) = *v_guard {
                    format!("{}:{}", k, val)
                } else {
                    format!("{}:<nil>", k)
                }
            })
            .collect();

        format!("map[{}]", formatted.join(" "))
    } else {
        "map[]".to_string()
    }
}
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

fn format_any(value: &dyn Any) -> String {
    format_any_at(value, false)
}

// Prints a value held inside the one being printed: a struct's interface
// field, an element of a []interface{}.
fn format_any_nested(value: &dyn Any) -> String {
    format_any_at(value, true)
}

fn format_any_at(value: &dyn Any, nested: bool) -> String {
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
    } else if let Some(v) = value.downcast_ref::<i8>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i16>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<isize>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u8>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u16>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<usize>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<Point>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<bool>>>>>>>>() {
        format_map(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>() {
        format_map(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Point>>>>() {
        go_format_any_pointer(nested, Rc::as_ptr(v) as *const (), || v.try_borrow().map(|__guard| __guard.is_none()).unwrap_or(false), || match (*v.borrow()).as_ref() { Some(__pointee) => format!("&{}", __pointee), None => "<nil>".to_string() })
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<String>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<f64>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>() {
        format_slice(v)
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
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
    if let Some(v) = value.downcast_ref::<IntList>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Point>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<bool>>>>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Box<dyn FnMut() -> i64>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Point>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<String>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<f64>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Registry>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Scale>() { return Box::new(v.clone()) as Box<dyn Any>; }

    panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>")
}

fn __go_type_name(val: &dyn Any) -> &'static str {
    if val.is::<i64>() { return "int" }
    if val.is::<i32>() { return "int32" }
    if val.is::<i8>() { return "int8" }
    if val.is::<i16>() { return "int16" }
    if val.is::<u64>() { return "uint" }
    if val.is::<u32>() { return "uint32" }
    if val.is::<u8>() { return "uint8" }
    if val.is::<u16>() { return "uint16" }
    if val.is::<f64>() { return "float64" }
    if val.is::<f32>() { return "float32" }
    if val.is::<bool>() { return "bool" }
    if val.is::<String>() { return "string" }
    if val.is::<Vec<i64>>() { return "[]int" }
    if val.is::<Vec<i32>>() { return "[]int32" }
    if val.is::<Vec<f64>>() { return "[]float64" }
    if val.is::<Vec<String>>() { return "[]string" }
    if val.is::<Vec<bool>>() { return "[]bool" }
    if val.is::<IntList>() { return "main.IntList" }
    if val.is::<Point>() { return "main.Point" }
    if val.is::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<bool>>>>>>>>() { return "map[string]bool" }
    if val.is::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>() { return "map[string]int" }
    if val.is::<Rc<RefCell<Option<Box<dyn FnMut() -> i64>>>>>() { return "func() int" }
    if val.is::<Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64>>>>>() { return "func(int) int" }
    if val.is::<Rc<RefCell<Option<Point>>>>() { return "*main.Point" }
    if val.is::<Rc<RefCell<Option<Vec<String>>>>>() { return "[]string" }
    if val.is::<Rc<RefCell<Option<Vec<f64>>>>>() { return "[]float64" }
    if val.is::<Rc<RefCell<Option<Vec<i64>>>>>() { return "[]int" }
    if val.is::<Registry>() { return "main.Registry" }
    if val.is::<Scale>() { return "main.Scale" }
    std::any::type_name_of_val(val)
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: Rc<RefCell<Option<i64>>>,
    pub y: Rc<RefCell<Option<i64>>>,
}

impl Point {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.y.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
            y: __go_clone_0_1,
        }
    }
}


impl Default for Point {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_0_1 = Rc::new(RefCell::new(Some(0)));
        Self {
            x: __go_default_0_0,
            y: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.y.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone, Default)]
pub struct IntList(pub Rc<RefCell<Option<Vec<i64>>>>);

impl Display for IntList {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


#[derive(Debug, Clone, Default)]
pub struct Registry(pub Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>);

impl Display for Registry {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_map(&self.0))
    }
}


pub type Scale = Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64>>>>;


#[derive(Clone, Default)]
pub struct Holder {
    pub v: Rc<RefCell<Option<Box<dyn Any>>>>,
}

impl Holder {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.v.clone();
        Self {
            v: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for Holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", match self.v.borrow().as_ref() { Some(__v) => format_any_nested(__v.as_ref()), None => "<nil>".to_string() });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Clone, Default)]
pub struct Pair {
    pub left: Rc<RefCell<Option<Box<dyn Any>>>>,
    pub right: Rc<RefCell<Option<Box<dyn Any>>>>,
}

impl Pair {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.left.clone();
        let __go_clone_0_1 = self.right.clone();
        Self {
            left: __go_clone_0_0,
            right: __go_clone_0_1,
        }
    }
}

impl std::fmt::Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", match self.left.borrow().as_ref() { Some(__v) => format_any_nested(__v.as_ref()), None => "<nil>".to_string() });
        let __go_fmt_1 = format!("{}", match self.right.borrow().as_ref() { Some(__v) => format_any_nested(__v.as_ref()), None => "<nil>".to_string() });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Shelf {
    pub items: Rc<RefCell<Option<IntList>>>,
}

impl Shelf {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.items.clone();
        Self {
            items: __go_clone_0_0,
        }
    }
}


impl Default for Shelf {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Default::default())));
        Self {
            items: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Shelf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.items.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Holder {
    pub fn set(&mut self, v: Rc<RefCell<Option<Box<dyn Any>>>>) {
        { let new_val = v.clone(); self.v = new_val; };
    }
}

/// classify reports the dynamic type of a boxed value. A pointer, slice, map
/// or function has to come back out of the interface as the same kind of thing
/// that went in.
pub fn classify(value: Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<String>>> {
    {
    let _ts_subject = value.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| {
        let mut __any = __v.as_ref() as &dyn Any;
        while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any>>() {
            __any = __boxed.as_ref() as &dyn Any;
        }
        __any
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<i64>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<i64>()).unwrap().clone())));
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("int {}", { let __v = (*v.borrow().as_ref().unwrap()).clone(); __v }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<String>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<String>()).unwrap().clone())));
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("{}{}", "string ".to_string(), (*v.borrow().as_ref().unwrap())))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>()).is_some() {
        let v = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>()).unwrap().clone();
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("[]int len {}", (*v.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Vec<String>>>>>()).is_some() {
        let v = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Vec<String>>>>>()).unwrap().clone();
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("[]string len {}", (*v.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>()).is_some() {
        let v = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>()).unwrap().clone();
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("map len {}", (*v.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Point>>>>()).is_some() {
        let v = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Point>>>>()).unwrap().clone();
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("*Point {}", (*(*v.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Point>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Point>()).unwrap().clone())));
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("Point {}", (*(*v.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<IntList>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<IntList>()).unwrap().clone())));
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("IntList len {}", { let __slice_holder = { let __named_slice = (*v.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Registry>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Registry>()).unwrap().clone())));
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("Registry len {}", { let __named_map_holder = v.clone(); let __named_map_guard = __named_map_holder.borrow(); let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()); drop(__named_map_guard); __map_holder.as_ref().map(|__map_holder| { let __map_guard = __map_holder.borrow(); __map_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }).unwrap_or(0) }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Box<dyn FnMut() -> i64>>>>>()).is_some() {
        let v = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Box<dyn FnMut() -> i64>>>>>()).unwrap().clone();
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("func {}", { let __f_ptr: *mut Box<dyn FnMut() -> i64> = { let mut __f_guard = v.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i64> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }))));;
    } else if _ts_is_nil {
        let v = _ts_subject.clone();
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some("nil".to_string())));;
    } else {
        let v = _ts_subject.clone();
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("other {}", __go_type_name(&**v.borrow().as_ref().unwrap())))));;
    }
    }
    unreachable!()
}

pub fn type_name(value: Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some(format!("{}", __go_type_name(&**value.borrow().as_ref().unwrap())))))
}

pub fn box_slice(xs: Rc<RefCell<Option<Vec<i64>>>>) -> Rc<RefCell<Option<Box<dyn Any>>>> {
    Rc::new(RefCell::new(Some(Box::new(xs.clone()) as Box<dyn Any>)))
}

pub fn box_with_error(xs: Rc<RefCell<Option<Vec<i64>>>>) -> (Rc<RefCell<Option<Box<dyn Any>>>>, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
    (Rc::new(RefCell::new(Some(Box::new(xs.clone()) as Box<dyn Any>))), Rc::new(RefCell::new(None)))
}

pub fn box_named_result(xs: Rc<RefCell<Option<Vec<i64>>>>) -> Rc<RefCell<Option<Box<dyn Any>>>> {
    let mut result: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(None));

    { let new_val = Box::new(xs.clone()) as Box<dyn Any>; *result.borrow_mut() = Some(new_val); };
    result.clone()
}

pub fn classify_all(args: Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> Rc<RefCell<Option<String>>> {
    let mut out = Rc::new(RefCell::new(Some("".to_string())));
    { let __range_holder = args.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.iter().map(|__e| go_any_clone(__e.as_ref())).collect::<Vec<_>>()).unwrap_or_default(); drop(__range_guard); for arg in __range_values.iter() {
        {
    let _ts_ref = arg;
    let _ts_is_nil = false;
    let _ts_val: Option<&dyn Any> = Some(_ts_ref.as_ref() as &dyn Any);
    if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>()).is_some() {
        { (*out.borrow_mut().as_mut().unwrap()).push_str(&"[]int ".to_string()); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>()).is_some() {
        { (*out.borrow_mut().as_mut().unwrap()).push_str(&"map ".to_string()); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Point>>>>()).is_some() {
        { (*out.borrow_mut().as_mut().unwrap()).push_str(&"*Point ".to_string()); };;
    } else {
        { (*out.borrow_mut().as_mut().unwrap()).push_str(&"other ".to_string()); };;
    }
    }
    } }
    return { let __owned = out.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) };
}

pub fn box_pointer(p: Rc<RefCell<Option<Point>>>) -> Rc<RefCell<Option<Box<dyn Any>>>> {
    Rc::new(RefCell::new(Some(Box::new(p.clone()) as Box<dyn Any>)))
}

pub fn sum(xs: Rc<RefCell<Option<Vec<i64>>>>) -> i64 {
    let mut total = Rc::new(RefCell::new(Some(0i64)));
    { let __range_holder = xs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter().copied() {
        { let __rhs = x; let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    return (*total.borrow().as_ref().unwrap());
}

fn main() {
    let mut xs = Rc::new(RefCell::new(Some(vec![1i64, 2i64, 3i64])));
    let mut names = Rc::new(RefCell::new(Some(vec!["a".to_string(), "b".to_string()])));
    let mut counts = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(1i64))))]))));
    let mut p = Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(7))), y: Rc::new(RefCell::new(Some(8))), ..Default::default() })));
    let mut pv = Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(9))), y: Rc::new(RefCell::new(Some(10))), ..Default::default() })));
    let mut list = Rc::new(RefCell::new(Some(IntList(Rc::new(RefCell::new(Some(vec![4i64, 5i64])))))));
    let mut reg = Rc::new(RefCell::new(Some(Registry(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("x".to_string(), Rc::new(RefCell::new(Some(1i64)))), ("y".to_string(), Rc::new(RefCell::new(Some(2i64))))]))))))));

        // Every way of putting a value into an interface.
    println!("{} {} {} {} {} {}", format!("{}", "argument:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(xs.clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(names.clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(counts.clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    println!("{} {} {} {} {} {}", format!("{}", "argument:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(p.clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new({ let __arg_holder = pv.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Box::new(move || -> i64 {
        5
    }) as Box<dyn FnMut() -> i64>))).clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    println!("{} {} {} {} {} {}", format!("{}", "literal:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(vec![4i64, 5i64]))).clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(2))), ..Default::default() }))).clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([])))).clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    println!("{} {} {} {} {} {}", format!("{}", "defined:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new((*list.borrow()).clone().unwrap_or_default()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new((*reg.borrow()).clone().unwrap_or_default()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(IntList(Rc::new(RefCell::new(Some(vec![6i64])))).clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    println!("{} {} {} {} {} {}", format!("{}", "scalar:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(42i64) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new("hi".to_string()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(3.5) as Box<dyn Any>)))).borrow().as_ref().unwrap())));

    let mut declared: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(xs.clone()) as Box<dyn Any>)));
    println!("{} {}", format!("{}", "declared:".to_string()), format!("{}", (*classify(declared.clone()).borrow().as_ref().unwrap())));
    let mut assigned: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(None));
    println!("{} {}", format!("{}", "unset:".to_string()), format!("{}", (*classify(assigned.clone()).borrow().as_ref().unwrap())));
    { let new_val = Box::new(counts.clone()) as Box<dyn Any>; *assigned.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", "assigned:".to_string()), format!("{}", (*classify(assigned.clone()).borrow().as_ref().unwrap())));
    { let new_val = Box::new(p.clone()) as Box<dyn Any>; *assigned.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", "assigned:".to_string()), format!("{}", (*classify(assigned.clone()).borrow().as_ref().unwrap())));
    { let new_val = Box::new({ let __arg_holder = pv.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Any>; *assigned.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", "assigned:".to_string()), format!("{}", (*classify(assigned.clone()).borrow().as_ref().unwrap())));
    println!("{} {} {} {}", format!("{}", "returned:".to_string()), format!("{}", (*classify(box_slice(xs.clone()).clone()).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(box_pointer(p.clone()).clone()).borrow().as_ref().unwrap())));

    let (mut both, _) = box_with_error(xs.clone());
    println!("{} {} {} {}", format!("{}", "returned with error:".to_string()), format!("{}", (*classify(both.clone()).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(box_named_result(xs.clone()).clone()).borrow().as_ref().unwrap())));
    println!("{} {} {} {} {} {}", format!("{}", "converted:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(xs.clone()) as Box<dyn Any>))).clone()).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(counts.clone()) as Box<dyn Any>))).clone()).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(p.clone()) as Box<dyn Any>))).clone()).borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "variadic:".to_string()), format!("{}", (*classify_all(Rc::new(RefCell::new(Some(vec![Box::new(xs.clone()) as Box<dyn Any>, Box::new(p.clone()) as Box<dyn Any>, Box::new(counts.clone()) as Box<dyn Any>, Box::new(1i64) as Box<dyn Any>])))).borrow().as_ref().unwrap())));
    let mut pair = Rc::new(RefCell::new(Some(Pair { left: Rc::new(RefCell::new(Some(Box::new(xs.clone()) as Box<dyn Any>))), right: Rc::new(RefCell::new(Some(Box::new(p.clone()) as Box<dyn Any>))), ..Default::default() })));
    println!("{} {} {} {}", format!("{}", "positional field:".to_string()), format!("{}", (*classify((*pair.borrow().as_ref().unwrap()).left.clone()).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify((*pair.borrow().as_ref().unwrap()).right.clone()).borrow().as_ref().unwrap())));
    let mut viaMethod = Rc::new(RefCell::new(Some(Holder { v: Default::default() })));
    (*viaMethod.borrow_mut().as_mut().unwrap()).set(Rc::new(RefCell::new(Some(Box::new(counts.clone()) as Box<dyn Any>))));
    println!("{} {}", format!("{}", "method argument:".to_string()), format!("{}", (*classify((*viaMethod.borrow().as_ref().unwrap()).v.clone()).borrow().as_ref().unwrap())));
    let xs_closure_clone = xs.clone(); let mut produce = Rc::new(RefCell::new(Some(Box::new(move || -> Rc<RefCell<Option<Box<dyn Any>>>> {
        return Rc::new(RefCell::new(Some(Box::new(xs_closure_clone.clone()) as Box<dyn Any>)));
    }) as Box<dyn FnMut() -> Rc<RefCell<Option<Box<dyn Any>>>>>)));
    println!("{} {}", format!("{}", "closure result:".to_string()), format!("{}", (*classify({ let __f_ptr: *mut Box<dyn FnMut() -> Rc<RefCell<Option<Box<dyn Any>>>>> = { let mut __f_guard = produce.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Rc<RefCell<Option<Box<dyn Any>>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }.clone()).borrow().as_ref().unwrap())));

        // A nil value of a defined slice type is still a value of that type.
    let mut noList: Rc<RefCell<Option<IntList>>> = Rc::new(RefCell::new(Some(Default::default())));
    let mut shelf = Rc::new(RefCell::new(Some(Shelf { items: Rc::new(RefCell::new(Some(Default::default()))) })));
    println!("{} {} {} {}", format!("{}", "nil defined:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new((*noList.borrow()).clone().unwrap_or_default()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new((*(*shelf.borrow().as_ref().unwrap()).items.borrow()).clone().unwrap_or_default()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    { let new_val = IntList(Rc::new(RefCell::new(Some(vec![1i64])))); *(*shelf.borrow().as_ref().unwrap()).items.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", "defined field:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new((*(*shelf.borrow().as_ref().unwrap()).items.borrow()).clone().unwrap_or_default()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));

    let mut holder = Rc::new(RefCell::new(Some(Holder { v: Rc::new(RefCell::new(Some(Box::new(xs.clone()) as Box<dyn Any>))), ..Default::default() })));
    println!("{} {}", format!("{}", "field:".to_string()), format!("{}", (*classify((*holder.borrow().as_ref().unwrap()).v.clone()).borrow().as_ref().unwrap())));
    { let new_val = Box::new(p.clone()) as Box<dyn Any>; *(*holder.borrow().as_ref().unwrap()).v.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", "field:".to_string()), format!("{}", (*classify((*holder.borrow().as_ref().unwrap()).v.clone()).borrow().as_ref().unwrap())));

    let mut byName = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Box<dyn Any>>>>>::from([("xs".to_string(), Rc::new(RefCell::new(Some(Box::new(xs.clone()) as Box<dyn Any>)))), ("p".to_string(), Rc::new(RefCell::new(Some(Box::new(p.clone()) as Box<dyn Any>))))]))));
    println!("{} {} {} {}", format!("{}", "map value:".to_string()), format!("{}", (*classify({ let __map_holder = byName.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"xs".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone()).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify({ let __map_holder = byName.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"p".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone()).borrow().as_ref().unwrap())));

    let mut items = Rc::new(RefCell::new(Some(vec![Box::new(xs.clone()) as Box<dyn Any>, Box::new(counts.clone()) as Box<dyn Any>, Box::new(p.clone()) as Box<dyn Any>, Box::new((*pv.borrow().as_ref().unwrap()).clone()) as Box<dyn Any>, Box::new((*list.borrow()).clone().unwrap_or_default()) as Box<dyn Any>, Box::new(7i64) as Box<dyn Any>])));
    { let __range_holder = items.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.iter().map(|__e| go_any_clone(__e.as_ref())).collect::<Vec<_>>()).unwrap_or_default(); drop(__range_guard); for item in __range_values.iter() {
        {
    let _ts_ref = item;
    let _ts_is_nil = false;
    let _ts_val: Option<&dyn Any> = Some(_ts_ref.as_ref() as &dyn Any);
    if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>()).is_some() {
        let v = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>()).unwrap().clone();
        println!("{} {}", format!("{}", "element: []int".to_string()), format!("{}", (*v.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>()).is_some() {
        let v = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>()).unwrap().clone();
        println!("{} {}", format!("{}", "element: map".to_string()), format!("{}", (*v.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Point>>>>()).is_some() {
        let v = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Point>>>>()).unwrap().clone();
        println!("{} {}", format!("{}", "element: *Point".to_string()), format!("{}", (*(*v.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Point>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Point>()).unwrap().clone())));
        println!("{} {}", format!("{}", "element: Point".to_string()), format!("{}", (*(*v.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<IntList>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<IntList>()).unwrap().clone())));
        println!("{} {}", format!("{}", "element: IntList".to_string()), format!("{}", { let __slice_holder = { let __named_slice = (*v.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }));;
    } else {
        let v = _ts_val.unwrap();
        println!("{}", format!("{}", "element: other".to_string()));;
    }
    }
    } }
    { let new_val = { let __append_target = items.clone(); let __append_value = Box::new(names.clone()) as Box<dyn Any>; (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(__append_value); __append_target.clone() }; items = new_val; };
    let mut appended = Rc::new(RefCell::new(Some(({
        let __idx = (((*items.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64) - (1 as i64)) as usize;
        let __seq_holder = items.clone();
        let __seq_guard = __seq_holder.borrow();
        let any_val = __seq_guard.as_ref().expect("nil []any in type assertion")[__idx].as_ref();
        { let __handle = any_val.downcast_ref::<Rc<RefCell<Option<Vec<String>>>>>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(any_val), "[]string")).clone(); let __value = (*__handle.borrow()).clone().unwrap_or_default(); __value }
    }))));
    println!("{} {} {}", format!("{}", "appended:".to_string()), format!("{}", (*appended.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*appended.borrow().as_ref().unwrap())[(0) as usize].clone()));

        // Assertions, in both forms.
    {
        let (mut got, mut ok) = ({
        let val = declared.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>() {
                (typed_val.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<Vec<i64>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Vec<i64>>)), false)
        }
    });;
        if ok {
            println!("{} {} {}", format!("{}", "comma-ok slice:".to_string()), format!("{}", (*got.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*got.borrow().as_ref().unwrap())[(0) as usize].clone()));;
        }
    }
    {
        let (_, mut ok) = ({
        let val = declared.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Vec<String>>>>>() {
                (typed_val.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<Vec<String>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Vec<String>>)), false)
        }
    });;
        if !ok {
            println!("{}", format!("{}", "comma-ok slice: not []string".to_string()));;
        }
    }
    {
        let (_, mut ok) = ({
        let val = declared.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<IntList>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), true)
            } else {
                (Rc::new(RefCell::new(Some(Default::default()))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(Default::default()))), false)
        }
    });;
        if !ok {
            println!("{}", format!("{}", "comma-ok slice: not IntList".to_string()));;
        }
    }
    let mut boxedMap: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(counts.clone()) as Box<dyn Any>)));
    {
        let (mut got, mut ok) = ({
        let val = boxedMap.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>() {
                (typed_val.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<BTreeMap<String, Rc<RefCell<Option<i64>>>>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<BTreeMap<String, Rc<RefCell<Option<i64>>>>>)), false)
        }
    });;
        if ok {
            println!("{} {}", format!("{}", "comma-ok map:".to_string()), format!("{}", { let __map_holder = got.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));;
        }
    }
    let mut boxedPointer: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(p.clone()) as Box<dyn Any>)));
    {
        let (mut got, mut ok) = ({
        let val = boxedPointer.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Point>>>>() {
                (typed_val.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<Point>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Point>)), false)
        }
    });;
        if ok {
            println!("{} {}", format!("{}", "comma-ok pointer:".to_string()), format!("{}", (*(*got.borrow().as_ref().unwrap()).y.borrow().as_ref().unwrap())));;
        }
    }
    {
        let (_, mut ok) = ({
        let val = boxedPointer.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Point>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), true)
            } else {
                (Rc::new(RefCell::new(Some(Default::default()))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(Default::default()))), false)
        }
    });;
        if !ok {
            println!("{}", format!("{}", "comma-ok pointer: not Point".to_string()));;
        }
    }
    let mut boxedList: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new((*list.borrow()).clone().unwrap_or_default()) as Box<dyn Any>)));
    {
        let (mut got, mut ok) = ({
        let val = boxedList.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<IntList>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), true)
            } else {
                (Rc::new(RefCell::new(Some(Default::default()))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(Default::default()))), false)
        }
    });;
        if ok {
            println!("{} {}", format!("{}", "comma-ok defined:".to_string()), format!("{}", { let __slice_holder = { let __named_slice = (*got.borrow().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }));;
        }
    }
    {
        let (_, mut ok) = ({
        let val = boxedList.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>() {
                (typed_val.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<Vec<i64>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Vec<i64>>)), false)
        }
    });;
        if !ok {
            println!("{}", format!("{}", "comma-ok defined: not []int".to_string()));;
        }
    }

    let mut plain = ({ let val = declared.clone(); let guard = val.borrow(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } });
    println!("{} {} {} {}", format!("{}", "single slice:".to_string()), format!("{}", (*plain.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*plain.borrow().as_ref().unwrap())[(2) as usize].clone()), format!("{}", sum(Rc::new(RefCell::new(Some(({
        let val = declared.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            { let __handle = any_val.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "[]int")).clone(); let __value = (*__handle.borrow()).clone().unwrap_or_default(); __value }
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "[]int")
        }
    })))))));
    let mut lookup = ({ let val = boxedMap.clone(); let guard = val.borrow(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } });
    println!("{} {} {}", format!("{}", "single map:".to_string()), format!("{}", (*lookup.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", { let __map_holder = lookup.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));
    let mut first = Rc::new(RefCell::new(Some(({
        let __idx = (0) as usize;
        let __seq_holder = items.clone();
        let __seq_guard = __seq_holder.borrow();
        let any_val = __seq_guard.as_ref().expect("nil []any in type assertion")[__idx].as_ref();
        { let __handle = any_val.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(any_val), "[]int")).clone(); let __value = (*__handle.borrow()).clone().unwrap_or_default(); __value }
    }))));
    let mut second = Rc::new(RefCell::new(Some(({
        let __idx = (1) as usize;
        let __seq_holder = items.clone();
        let __seq_guard = __seq_holder.borrow();
        let any_val = __seq_guard.as_ref().expect("nil []any in type assertion")[__idx].as_ref();
        { let __handle = any_val.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(any_val), "map[string]int")).clone(); let __value = (*__handle.borrow()).clone().unwrap_or_default(); __value }
    }))));
    let mut third = ({
        let __idx = (2) as usize;
        let __seq_holder = items.clone();
        let __seq_guard = __seq_holder.borrow();
        let any_val = __seq_guard.as_ref().expect("nil []any in type assertion")[__idx].as_ref();
        any_val.downcast_ref::<Rc<RefCell<Option<Point>>>>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(any_val), "*main.Point")).clone()
    }).clone();
    println!("{} {} {} {}", format!("{}", "single element:".to_string()), format!("{}", (*first.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", { let __map_holder = second.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", (*(*third.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));
    let mut empty: Rc<RefCell<Option<Vec<i64>>>> = Rc::new(RefCell::new(None));
    let mut boxedEmpty: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(empty.clone()) as Box<dyn Any>)));
    let mut nothing = ({ let val = boxedEmpty.clone(); let guard = val.borrow(); if let Some(ref any_val) = *guard { any_val.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>().expect("type assertion failed").clone() } else { panic!("type assertion on nil interface") } });
    println!("{} {}", format!("{}", "single nil slice:".to_string()), format!("{}", (*nothing.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));

        // The boxed value is the same slice and the same pointee, not a copy.
    {
        let (mut shared, mut ok) = ({
        let val = declared.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>() {
                (typed_val.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<Vec<i64>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Vec<i64>>)), false)
        }
    });;
        if ok {
            { let __idx = (0) as usize; let __rhs = 99; (*shared.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };;
        }
    }
    println!("{} {}", format!("{}", "shared slice:".to_string()), format!("{}", (*xs.borrow().as_ref().unwrap())[(0) as usize].clone()));
    {
    let _ts_subject = boxedPointer.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| {
        let mut __any = __v.as_ref() as &dyn Any;
        while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any>>() {
            __any = __boxed.as_ref() as &dyn Any;
        }
        __any
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Point>>>>()).is_some() {
        let shared = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Point>>>>()).unwrap().clone();
        drop(_ts_guard);
        { let new_val = 70; *(*shared.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };;
    }
    }
    println!("{} {}", format!("{}", "shared pointer:".to_string()), format!("{}", (*(*p.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));

        // A function value is called through the interface, and keeps its state.
    let mut scale: Scale = Rc::new(RefCell::new(Some(Box::new(move |x: Rc<RefCell<Option<i64>>>| -> i64 {
        (*x.borrow().as_ref().unwrap()) * 10
    }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64>)));
    let mut boxedScale: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(scale.clone()) as Box<dyn Any>)));
    println!("{} {}", format!("{}", "call defined func:".to_string()), format!("{}", ({
        let __f_holder = { let val = boxedScale.clone(); let guard = val.borrow(); match guard.as_ref() { Some(any_val) => any_val.downcast_ref::<Scale>().expect("type assertion failed").clone(), None => panic!("type assertion on nil interface") } };
        (|__f_arg0| {
            let mut __f_guard = __f_holder.borrow_mut();
            let __f: &mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64> = __f_guard.as_mut().expect("call of nil function");
            let __f_result = (*__f)(__f_arg0);
            __f_result
        })(Rc::new(RefCell::new(Some(4))))
    })));
    let mut calls = Rc::new(RefCell::new(Some(0i64)));
    let mut calls_closure_clone = calls.clone(); let mut counter: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Box::new(move || -> i64 {
        { let mut guard = calls_closure_clone.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return (*calls_closure_clone.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut() -> i64>))).clone()) as Box<dyn Any>)));
    ({
        let __f_holder = { let val = counter.clone(); let guard = val.borrow(); match guard.as_ref() { Some(any_val) => any_val.downcast_ref::<Rc<RefCell<Option<Box<dyn FnMut() -> i64>>>>>().expect("type assertion failed").clone(), None => panic!("type assertion on nil interface") } };
        (|| {
            let mut __f_guard = __f_holder.borrow_mut();
            let __f: &mut Box<dyn FnMut() -> i64> = __f_guard.as_mut().expect("call of nil function");
            let __f_result = (*__f)();
            __f_result
        })()
    });
    println!("{} {} {}", format!("{}", "call func:".to_string()), format!("{}", ({
        let __f_holder = { let val = counter.clone(); let guard = val.borrow(); match guard.as_ref() { Some(any_val) => any_val.downcast_ref::<Rc<RefCell<Option<Box<dyn FnMut() -> i64>>>>>().expect("type assertion failed").clone(), None => panic!("type assertion on nil interface") } };
        (|| {
            let mut __f_guard = __f_holder.borrow_mut();
            let __f: &mut Box<dyn FnMut() -> i64> = __f_guard.as_mut().expect("call of nil function");
            let __f_result = (*__f)();
            __f_result
        })()
    })), format!("{}", { let __v = (*calls.borrow().as_ref().unwrap()).clone(); __v }));

        // An argument may itself call the same boxed function. Go finishes the
        // inner call before the outer one starts.
    let mut step: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Box::new(move |n: Rc<RefCell<Option<i64>>>| -> i64 {
        println!("{} {}", format!("{}", "step".to_string()), format!("{}", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v }));
        (*n.borrow().as_ref().unwrap()) + 1
    }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64>))).clone()) as Box<dyn Any>)));
    println!("{} {}", format!("{}", "nested call:".to_string()), format!("{}", ({
        let __f_holder = { let val = step.clone(); let guard = val.borrow(); match guard.as_ref() { Some(any_val) => any_val.downcast_ref::<Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64>>>>>().expect("type assertion failed").clone(), None => panic!("type assertion on nil interface") } };
        (|__f_arg0| {
            let mut __f_guard = __f_holder.borrow_mut();
            let __f: &mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64> = __f_guard.as_mut().expect("call of nil function");
            let __f_result = (*__f)(__f_arg0);
            __f_result
        })(Rc::new(RefCell::new(Some(({
        let __f_holder = { let val = step.clone(); let guard = val.borrow(); match guard.as_ref() { Some(any_val) => any_val.downcast_ref::<Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64>>>>>().expect("type assertion failed").clone(), None => panic!("type assertion on nil interface") } };
        (|__f_arg0| {
            let mut __f_guard = __f_holder.borrow_mut();
            let __f: &mut Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64> = __f_guard.as_mut().expect("call of nil function");
            let __f_result = (*__f)(__f_arg0);
            __f_result
        })(Rc::new(RefCell::new(Some(1))))
    })))))
    })));

        // An element of a slice of slices, and the loop variable over one, is
        // still a []int when it is put in an interface. So is a map's value.
    let mut matrix = Rc::new(RefCell::new(Some(vec![vec![1i64], vec![2i64, 3i64]])));
    println!("{} {}", format!("{}", "element:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some((*matrix.borrow().as_ref().unwrap())[(1) as usize].clone().clone())))) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    { let __range_holder = matrix.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for row in __range_values.iter() {
        println!("{} {}", format!("{}", "range value:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some((*row).clone())))) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    } }
    let mut firstRow: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some((*matrix.borrow().as_ref().unwrap())[(0) as usize].clone().clone())))) as Box<dyn Any>)));
    {
        let (mut row, mut ok) = ({
        let val = firstRow.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>() {
                (typed_val.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<Vec<i64>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Vec<i64>>)), false)
        }
    });;
        if ok {
            println!("{} {}", format!("{}", "asserted element:".to_string()), format!("{}", (*row.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));;
        }
    }
    let mut rowsByName = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Vec<i64>>>>>::from([("k".to_string(), Rc::new(RefCell::new(Some(vec![4i64, 5i64, 6i64]))))]))));
    println!("{} {}", format!("{}", "slice in map:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new({ let __map_holder = rowsByName.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"k".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    for (_, value) in { let __range_holder = rowsByName.clone(); let __range_guard = __range_holder.borrow(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        println!("{} {}", format!("{}", "slice in map, range value:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(value.clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    }
    let mut tables = Rc::new(RefCell::new(Some(vec![BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(1i64)))), ("b".to_string(), Rc::new(RefCell::new(Some(2i64))))])])));
    println!("{} {}", format!("{}", "slice of maps:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some((*tables.borrow().as_ref().unwrap())[(0) as usize].clone().clone())))) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    { let __range_holder = tables.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for table in __range_values.iter() {
        println!("{} {}", format!("{}", "range over maps:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some((*table).clone())))) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    } }

        // %T names the dynamic type, and the static type of a typed variable.
    println!("{} {} {} {}", format!("{}", "dynamic:".to_string()), format!("{}", (*type_name(Rc::new(RefCell::new(Some(Box::new(xs.clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", (*type_name(Rc::new(RefCell::new(Some(Box::new(names.clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", (*type_name(Rc::new(RefCell::new(Some(Box::new(counts.clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    println!("{} {} {} {}", format!("{}", "dynamic:".to_string()), format!("{}", (*type_name(Rc::new(RefCell::new(Some(Box::new(p.clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", (*type_name(Rc::new(RefCell::new(Some(Box::new({ let __arg_holder = pv.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", (*type_name(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(vec![1.0]))).clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    println!("{} {} {} {} {}", format!("{}", "dynamic:".to_string()), format!("{}", (*type_name(Rc::new(RefCell::new(Some(Box::new(7i64) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", (*type_name(Rc::new(RefCell::new(Some(Box::new("s".to_string()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", (*type_name(Rc::new(RefCell::new(Some(Box::new(true) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", (*type_name(Rc::new(RefCell::new(Some(Box::new(2.5) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    println!("{} {} {} {}", format!("{}", "unmatched:".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(vec![1.0]))).clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*classify(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<bool>>>>::from([])))).clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    let mut none: Rc<RefCell<Option<Vec<i64>>>> = Rc::new(RefCell::new(None));
    let mut nowhere: Rc<RefCell<Option<Point>>> = Rc::new(RefCell::new(None));
    print!("static: {} {} {} {} {}\n", "[]int", "map[string]int", "*main.Point", "main.Point", "main.IntList");
    print!("static nil: {} {}\n", "[]int", "*main.Point");
}