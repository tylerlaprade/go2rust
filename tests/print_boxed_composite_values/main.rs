use std::any::Any;
use std::cell::{RefCell};
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

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
    } else if let Some(v) = value.downcast_ref::<NotFound>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Point>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<Point>>>>>>>>() {
        format_map(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>>() {
        format_map(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<i32, Rc<RefCell<Option<String>>>>>>>>() {
        format_map(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Point>>>>() {
        match (*v.borrow()).as_ref() { Some(__pointee) => format!("&{}", __pointee), None => "<nil>".to_string() }
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<Point>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<String>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<bool>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<f64>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<i32>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Status>() {
        format!("{}", v)
    } else {
        "<unknown>".to_string()
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
    if let Some(v) = value.downcast_ref::<NotFound>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Point>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<Point>>>>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<i32, Rc<RefCell<Option<String>>>>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Point>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<Point>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<String>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<bool>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<f64>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<i32>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Status>() { return Box::new(v.clone()) as Box<dyn Any>; }

    panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>")
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: Rc<RefCell<Option<i32>>>,
    pub y: Rc<RefCell<Option<i32>>>,
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


#[derive(Debug, Clone)]
pub struct Status {
    pub code: Rc<RefCell<Option<i32>>>,
}

impl Status {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.code.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            code: __go_clone_0_0,
        }
    }
}


impl Default for Status {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            code: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct NotFound {
    pub name: Rc<RefCell<Option<String>>>,
}

impl NotFound {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for NotFound {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for NotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


impl Status {
    pub fn string(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("status {}", (*self.code.borrow().as_ref().unwrap())))))
    }
}

impl NotFound {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("{}{}", (*self.name.clone().borrow().as_ref().unwrap()), " not found".to_string()))));
    }
}

impl StdError for NotFound {}


/// show prints whatever it is given. The value arrives as an interface, so
/// the printer has to find out what it holds.
pub fn show(v: Rc<RefCell<Option<Box<dyn Any>>>>) {
    println!("{}", format!("{}", format_any(v.borrow().as_ref().unwrap().as_ref())));
}

fn main() {
    show(Rc::new(RefCell::new(Some(Box::new(42) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new("hi".to_string()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(2.5) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(true) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(vec![1, 2, 3]))).clone()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(vec!["a".to_string(), "b".to_string()]))).clone()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(vec![true, false]))).clone()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(vec![1.5, 2.0]))).clone()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(vec![Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(2))), ..Default::default() }, Point { x: Rc::new(RefCell::new(Some(3))), y: Rc::new(RefCell::new(Some(4))), ..Default::default() }]))).clone()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([("b".to_string(), Rc::new(RefCell::new(Some(2)))), ("a".to_string(), Rc::new(RefCell::new(Some(1))))])))).clone()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(BTreeMap::<i32, Rc<RefCell<Option<String>>>>::from([(2, Rc::new(RefCell::new(Some("two".to_string())))), (1, Rc::new(RefCell::new(Some("one".to_string()))))])))).clone()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Point>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(2))), ..Default::default() }))))])))).clone()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(2))), ..Default::default() }) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(3))), y: Rc::new(RefCell::new(Some(4))), ..Default::default() }))).clone()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Status { code: Rc::new(RefCell::new(Some(404))), ..Default::default() }) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(NotFound { name: Rc::new(RefCell::new(Some("x".to_string()))), ..Default::default() }) as Box<dyn Any>))));

    let mut none: Rc<RefCell<Option<Vec<i32>>>> = Rc::new(RefCell::new(None));
    show(Rc::new(RefCell::new(Some(Box::new(none.clone()) as Box<dyn Any>))));
    let mut nowhere: Rc<RefCell<Option<Point>>> = Rc::new(RefCell::new(None));
    show(Rc::new(RefCell::new(Some(Box::new(nowhere.clone()) as Box<dyn Any>))));

    let mut xs = Rc::new(RefCell::new(Some(vec![4, 5])));
    let mut p = Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(6))), y: Rc::new(RefCell::new(Some(7))), ..Default::default() })));
    let mut boxed: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(xs.clone()) as Box<dyn Any>)));
    println!("{} {} {}", format!("{}", "println:".to_string()), format!("{}", format_any(boxed.borrow().as_ref().unwrap().as_ref())), format!("{}", Point { x: Rc::new(RefCell::new(Some(5))), y: Rc::new(RefCell::new(Some(6))), ..Default::default() }));
    print!("printf: {} {}\n", format_any(boxed.borrow().as_ref().unwrap().as_ref()), 7);
    { let new_val = Box::new(p.clone()) as Box<dyn Any>; *boxed.borrow_mut() = Some(new_val); };
    print!("printf: {}\n", format_any(boxed.borrow().as_ref().unwrap().as_ref()));

    let mut items = Rc::new(RefCell::new(Some(vec![Box::new(Rc::new(RefCell::new(Some(vec![1, 2]))).clone()) as Box<dyn Any>, Box::new(Point { x: Rc::new(RefCell::new(Some(7))), y: Rc::new(RefCell::new(Some(8))), ..Default::default() }) as Box<dyn Any>, Box::new(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i32>>>>::from([("k".to_string(), Rc::new(RefCell::new(Some(1))))])))).clone()) as Box<dyn Any>, Box::new("s".to_string()) as Box<dyn Any>, Box::new(3) as Box<dyn Any>])));
    { let __range_holder = items.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.iter().map(|__e| go_any_clone(__e.as_ref())).collect::<Vec<_>>()).unwrap_or_default(); drop(__range_guard); for item in __range_values.iter() {
        println!("{} {}", format!("{}", "item:".to_string()), format!("{}", format_any(item.as_ref())));
    } }

    let mut names = Rc::new(RefCell::new(Some(vec!["x".to_string()])));
    let mut byName = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Box<dyn Any>>>>>::from([("point".to_string(), Rc::new(RefCell::new(Some(Box::new(Point { x: Rc::new(RefCell::new(Some(9))), y: Rc::new(RefCell::new(Some(9))), ..Default::default() }) as Box<dyn Any>)))), ("list".to_string(), Rc::new(RefCell::new(Some(Box::new(names.clone()) as Box<dyn Any>))))]))));
    println!("{} {}", format!("{}", format_any(({ let __map_holder = byName.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"point".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }).borrow().as_ref().unwrap().as_ref())), format!("{}", format_any(({ let __map_holder = byName.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"list".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }).borrow().as_ref().unwrap().as_ref())));
}