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
    } else if let Some(v) = value.downcast_ref::<Label>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Node>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<NotFound>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Outcome>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Point>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<Point>>>>>>>>() {
        format_map(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>>() {
        format_map(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<i32, Rc<RefCell<Option<String>>>>>>>>() {
        format_map(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Node>>>>() {
        go_format_any_pointer(nested, Rc::as_ptr(v) as *const (), || v.try_borrow().map(|__guard| __guard.is_none()).unwrap_or(false), || match (*v.borrow()).as_ref() { Some(__pointee) => format!("&{}", __pointee), None => "<nil>".to_string() })
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<NotFound>>>>() {
        match (*v.borrow()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() }
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Point>>>>() {
        go_format_any_pointer(nested, Rc::as_ptr(v) as *const (), || v.try_borrow().map(|__guard| __guard.is_none()).unwrap_or(false), || match (*v.borrow()).as_ref() { Some(__pointee) => format!("&{}", __pointee), None => "<nil>".to_string() })
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Status>>>>() {
        match (*v.borrow()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() }
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
    } else if let Some(v) = value.downcast_ref::<Report>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Status>() {
        format!("{}", v)
    } else {
        "<unknown>".to_string()
    }
}

fn format_any_slice_values(slice: &Rc<RefCell<Option<Vec<Box<dyn Any>>>>>, format_value: fn(&dyn Any) -> String) -> String {
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| format_value(v.as_ref())).collect();
        formatted.join(" ")
    } else {
        String::new()
    }
}

fn format_any_slice(slice: &Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> String {
    format!("[{}]", format_any_slice_values(slice, format_any_nested))
}

fn format_any_variadic(slice: &Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> String {
    format_any_slice_values(slice, format_any)
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
    if let Some(v) = value.downcast_ref::<Label>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Node>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<NotFound>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Outcome>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Point>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<Point>>>>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i32>>>>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<i32, Rc<RefCell<Option<String>>>>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Node>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<NotFound>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Point>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Status>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<Point>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<String>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<bool>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<f64>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<i32>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Report>() { return Box::new(v.clone()) as Box<dyn Any>; }
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


/// Node can point at itself through its interface field.
#[derive(Clone)]
pub struct Node {
    pub name: Rc<RefCell<Option<String>>>,
    pub next: Rc<RefCell<Option<Box<dyn Any>>>>,
}

impl Node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = self.next.clone();
        Self {
            name: __go_clone_0_0,
            next: __go_clone_1_0,
        }
    }
}


impl Default for Node {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        Self {
            name: __go_default_0_0,
            next: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", match self.next.borrow().as_ref() { Some(__v) => format_any_nested(__v.as_ref()), None => "<nil>".to_string() });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


/// Label prints itself by printing another value. That is a print call of its
/// own: the pointer it prints is at the top level of that call, wherever the
/// Label sits in the value being printed.
#[derive(Debug, Clone)]
pub struct Label {
    pub x: Rc<RefCell<Option<i32>>>,
}

impl Label {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
        }
    }
}


impl Default for Label {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            x: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().borrow().as_ref().unwrap()))
    }
}


/// Missing does the same from its Error method.
#[derive(Debug, Clone)]
pub struct Missing {
    pub line: Rc<RefCell<Option<i32>>>,
}

impl Missing {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.line.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            line: __go_clone_0_0,
        }
    }
}


impl Default for Missing {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            line: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Missing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


/// Outcome is the usual result struct: its error is nil unless something
/// failed.
#[derive(Clone)]
pub struct Outcome {
    pub value: Rc<RefCell<Option<i32>>>,
    pub err: Rc<RefCell<Option<Rc<dyn StdError>>>>,
}

impl Outcome {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = self.err.clone();
        Self {
            value: __go_clone_0_0,
            err: __go_clone_1_0,
        }
    }
}


impl Default for Outcome {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(None));
        Self {
            value: __go_default_0_0,
            err: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", match self.err.borrow().as_ref() { Some(__v) => format!("{}", __v), None => "<nil>".to_string() });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


/// Failure is another name for error. A field of this type is an error field.
pub type Failure = Rc<RefCell<Option<Rc<dyn StdError>>>>;


#[derive(Clone, Default)]
pub struct Report {
    pub err: Failure,
}

impl Report {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.err.clone();
        Self {
            err: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", match self.err.borrow().as_ref() { Some(__v) => format!("{}", __v), None => "<nil>".to_string() });
        write!(f, "{{{}}}", __go_fmt_0)
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


impl Label {
    pub fn string(&self) -> Rc<RefCell<Option<String>>> {
        render(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some({ let __selector_holder = self.x.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() }))).clone()) as Box<dyn Any>))))
    }
}

impl Missing {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("{}{}", "missing at ".to_string(), (*render(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some({ let __selector_holder = self.line.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() }))).clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())))))
    }
}

impl StdError for Missing {}


pub fn render(v: Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some(format!("{}", format_any(v.borrow().as_ref().unwrap().as_ref())))))
}

/// shape describes text around the addresses in it, which change from run to
/// run: whether it starts with prefix, and how many addresses it holds.
pub fn shape(text: Rc<RefCell<Option<String>>>, prefix: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some(format!("{} {}", (*Rc::new(RefCell::new(Some({ let __s = (*text.borrow().as_ref().unwrap()).clone(); let __arg = (*prefix.borrow().as_ref().unwrap()).clone(); __s.starts_with(&__arg) }))).borrow().as_ref().unwrap()), ((*Rc::new(RefCell::new(Some({ let __s = (*text.borrow().as_ref().unwrap()).clone(); let __sep = "0x".to_string(); __s.split(&__sep).map(|__part| __part.to_string()).collect::<Vec<String>>() }))).borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) - (1 as i32)))))
}

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

        // A pointer below the top level prints as its address, so a value that
        // points back to itself prints once and stops. The address changes from
        // run to run; check the shape around it.
    let mut r#loop = Rc::new(RefCell::new(Some(Node { name: Rc::new(RefCell::new(Some("loop".to_string()))), ..Default::default() })));
    { let new_val = Box::new(r#loop.clone()) as Box<dyn Any>; *(*r#loop.borrow().as_ref().unwrap()).next.borrow_mut() = Some(new_val); };
    let mut text = render(Rc::new(RefCell::new(Some(Box::new(r#loop.clone()) as Box<dyn Any>))));
    println!("{} {} {} {}", format!("{}", "cycle:".to_string()), format!("{}", (*Rc::new(RefCell::new(Some({ let __s = (*text.borrow().as_ref().unwrap()).clone(); let __arg = "&{loop 0x".to_string(); __s.starts_with(&__arg) }))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some({ let __s = (*text.borrow().as_ref().unwrap()).clone(); let __arg = "}".to_string(); __s.ends_with(&__arg) }))).borrow().as_ref().unwrap())), format!("{}", ((*Rc::new(RefCell::new(Some({ let __s = (*text.borrow().as_ref().unwrap()).clone(); let __sep = "loop".to_string(); __s.split(&__sep).map(|__part| __part.to_string()).collect::<Vec<String>>() }))).borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) - (1 as i32)));
    let mut tail = Rc::new(RefCell::new(Some(Node { name: Rc::new(RefCell::new(Some("tail".to_string()))), ..Default::default() })));
    { let new_val = render(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Node { name: Rc::new(RefCell::new(Some("head".to_string()))), next: Rc::new(RefCell::new(Some(Box::new(tail.clone()) as Box<dyn Any>))), ..Default::default() }))).clone()) as Box<dyn Any>)))); let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *text.borrow_mut() = __moved_val; };
    println!("{} {} {}", format!("{}", "chain:".to_string()), format!("{}", (*Rc::new(RefCell::new(Some({ let __s = (*text.borrow().as_ref().unwrap()).clone(); let __arg = "&{head 0x".to_string(); __s.starts_with(&__arg) }))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some({ let __s = (*text.borrow().as_ref().unwrap()).clone(); let __arg = "tail".to_string(); __s.contains(&__arg) }))).borrow().as_ref().unwrap())));

        // The same holds when the value being printed is a struct and the
        // pointer is in one of its fields, however the struct reaches the
        // printer, and when the pointer is an element of a slice of interfaces.
    println!("{} {}", format!("{}", "struct value, boxed:".to_string()), format!("{}", (*shape(render(Rc::new(RefCell::new(Some(Box::new({ let __v = (*r#loop.borrow().as_ref().unwrap()).clone(); __v }) as Box<dyn Any>)))), Rc::new(RefCell::new(Some("{loop 0x".to_string())))).borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "struct value, typed:".to_string()), format!("{}", (*shape(Rc::new(RefCell::new(Some(format!("{}", { let __v = (*r#loop.borrow().as_ref().unwrap()).clone(); __v })))), Rc::new(RefCell::new(Some("{loop 0x".to_string())))).borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "pointer, typed:".to_string()), format!("{}", (*shape(Rc::new(RefCell::new(Some(format!("{}", { let __text = match r#loop.borrow().as_ref() { Some(__value) => format!("&{}", __value), None => "<nil>".to_string() }; __text })))), Rc::new(RefCell::new(Some("&{loop 0x".to_string())))).borrow().as_ref().unwrap())));
    { let new_val = render(Rc::new(RefCell::new(Some(Box::new(Node { name: Rc::new(RefCell::new(Some("holder".to_string()))), next: Rc::new(RefCell::new(Some(Box::new(tail.clone()) as Box<dyn Any>))), ..Default::default() }) as Box<dyn Any>)))); let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *text.borrow_mut() = __moved_val; };
    println!("{} {} {}", format!("{}", "struct holding a pointer:".to_string()), format!("{}", (*shape(Rc::new(RefCell::new(Some({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some("{holder 0x".to_string())))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some({ let __s = (*text.borrow().as_ref().unwrap()).clone(); let __arg = "tail".to_string(); __s.contains(&__arg) }))).borrow().as_ref().unwrap())));
    let mut mixed = Rc::new(RefCell::new(Some(vec![Box::new(tail.clone()) as Box<dyn Any>, Box::new(1) as Box<dyn Any>])));
    { let new_val = Rc::new(RefCell::new(Some(format!("{}", format_any_slice(&mixed))))); let __moved_val = { let mut __guard = new_val.borrow_mut(); __guard.take() }; *text.borrow_mut() = __moved_val; };
    println!("{} {} {}", format!("{}", "slice holding a pointer:".to_string()), format!("{}", (*shape(Rc::new(RefCell::new(Some({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))), Rc::new(RefCell::new(Some("[0x".to_string())))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some({ let __s = (*text.borrow().as_ref().unwrap()).clone(); let __arg = " 1]".to_string(); __s.ends_with(&__arg) }))).borrow().as_ref().unwrap())));
    let mut noNode: Rc<RefCell<Option<Node>>> = Rc::new(RefCell::new(None));
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Node { name: Rc::new(RefCell::new(Some("end".to_string()))), next: Rc::new(RefCell::new(Some(Box::new(noNode.clone()) as Box<dyn Any>))), ..Default::default() }))).clone()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(p.clone()) as Box<dyn Any>))));

        // The zero value of a struct has nil interface and error fields. They
        // print as <nil>, whether the struct is printed directly or through an
        // interface.
    println!("{}", format!("{}", Node { name: Rc::new(RefCell::new(Some("typed".to_string()))), ..Default::default() }));
    show(Rc::new(RefCell::new(Some(Box::new(Node { name: Rc::new(RefCell::new(Some("boxed".to_string()))), ..Default::default() }) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Node { name: Rc::new(RefCell::new(Some("pointer".to_string()))), ..Default::default() }))).clone()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Node { name: Rc::new(RefCell::new(Some("full".to_string()))), next: Rc::new(RefCell::new(Some(Box::new(3) as Box<dyn Any>))), ..Default::default() }) as Box<dyn Any>))));
    println!("{}", format!("{}", Outcome { value: Rc::new(RefCell::new(Some(1))), ..Default::default() }));
    show(Rc::new(RefCell::new(Some(Box::new(Outcome { value: Rc::new(RefCell::new(Some(2))), ..Default::default() }) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Outcome { value: Rc::new(RefCell::new(Some(3))), err: Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(format!("failed")))))), ..Default::default() }) as Box<dyn Any>))));
    println!("{}", format!("{}", Report { err: Default::default() }));
    show(Rc::new(RefCell::new(Some(Box::new(Report { err: Default::default() }) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Report { err: Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(format!("reported")))))), ..Default::default() }) as Box<dyn Any>))));

        // A String or Error method that prints starts a print call of its own.
        // The pointer it prints is expanded, however deep the value with the
        // method sits: in a field, behind a pointer, in a slice, in an error field.
    println!("{}", format!("{}", Label { x: Rc::new(RefCell::new(Some(7))), ..Default::default() }));
    show(Rc::new(RefCell::new(Some(Box::new(Label { x: Rc::new(RefCell::new(Some(7))), ..Default::default() }) as Box<dyn Any>))));
    println!("{}", format!("{}", Node { name: Rc::new(RefCell::new(Some("row".to_string()))), next: Rc::new(RefCell::new(Some(Box::new(Label { x: Rc::new(RefCell::new(Some(7))), ..Default::default() }) as Box<dyn Any>))), ..Default::default() }));
    show(Rc::new(RefCell::new(Some(Box::new(Node { name: Rc::new(RefCell::new(Some("row".to_string()))), next: Rc::new(RefCell::new(Some(Box::new(Label { x: Rc::new(RefCell::new(Some(7))), ..Default::default() }) as Box<dyn Any>))), ..Default::default() }) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Node { name: Rc::new(RefCell::new(Some("row".to_string()))), next: Rc::new(RefCell::new(Some(Box::new(Label { x: Rc::new(RefCell::new(Some(7))), ..Default::default() }) as Box<dyn Any>))), ..Default::default() }))).clone()) as Box<dyn Any>))));
    let mut labels = Rc::new(RefCell::new(Some(vec![Box::new(Label { x: Rc::new(RefCell::new(Some(8))), ..Default::default() }) as Box<dyn Any>, Box::new(Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(2))), y: Rc::new(RefCell::new(Some(3))), ..Default::default() }))).clone()) as Box<dyn Any>, Box::new(1) as Box<dyn Any>])));
    println!("{} {}", format!("{}", "slice:".to_string()), format!("{}", (*shape(Rc::new(RefCell::new(Some(format!("{}", format_any_slice(&labels))))), Rc::new(RefCell::new(Some("[&{8 0} 0x".to_string())))).borrow().as_ref().unwrap())));
    println!("{}", format_any_variadic(&labels));
    println!("{}", format!("{}", Outcome { value: Rc::new(RefCell::new(Some(4))), err: Rc::new(RefCell::new(Some(Rc::new(Missing { line: Rc::new(RefCell::new(Some(9))), ..Default::default() }) as Rc<dyn StdError>))), ..Default::default() }));
    show(Rc::new(RefCell::new(Some(Box::new(Node { name: Rc::new(RefCell::new(Some("outer".to_string()))), next: Rc::new(RefCell::new(Some(Box::new(Outcome { value: Rc::new(RefCell::new(Some(5))), err: Rc::new(RefCell::new(Some(Rc::new(Missing { line: Rc::new(RefCell::new(Some(9))), ..Default::default() }) as Rc<dyn StdError>))), ..Default::default() }) as Box<dyn Any>))), ..Default::default() }) as Box<dyn Any>))));

        // A pointer to a type with a String or Error method prints through the
        // method, at the top level and inside another value. A nil one is <nil>.
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Status { code: Rc::new(RefCell::new(Some(404))), ..Default::default() }))).clone()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Node { name: Rc::new(RefCell::new(Some("n".to_string()))), next: Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Status { code: Rc::new(RefCell::new(Some(5))), ..Default::default() }))).clone()) as Box<dyn Any>))), ..Default::default() }) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(NotFound { name: Rc::new(RefCell::new(Some("y".to_string()))), ..Default::default() }))).clone()) as Box<dyn Any>))));
    let mut noStatus: Rc<RefCell<Option<Status>>> = Rc::new(RefCell::new(None));
    show(Rc::new(RefCell::new(Some(Box::new(noStatus.clone()) as Box<dyn Any>))));
    show(Rc::new(RefCell::new(Some(Box::new(Node { name: Rc::new(RefCell::new(Some("n".to_string()))), next: Rc::new(RefCell::new(Some(Box::new(noStatus.clone()) as Box<dyn Any>))), ..Default::default() }) as Box<dyn Any>))));
}