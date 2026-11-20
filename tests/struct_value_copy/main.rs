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


#[derive(Debug, Clone)]
pub struct Line {
    pub a: Rc<RefCell<Option<Point>>>,
    pub b: Rc<RefCell<Option<Point>>>,
}

impl Line {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.a.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_0_1 = { let __guard = self.b.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            a: __go_clone_0_0,
            b: __go_clone_0_1,
        }
    }
}


impl Default for Line {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Point::default())));
        let __go_default_0_1 = Rc::new(RefCell::new(Some(Point::default())));
        Self {
            a: __go_default_0_0,
            b: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.a.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.b.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Grid {
    pub cells: Rc<RefCell<Option<[Point; 2]>>>,
}

impl Grid {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.cells.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.each_ref().map(|__go_elem| __go_elem.__go_value_clone())))) };
        Self {
            cells: __go_clone_0_0,
        }
    }
}


impl Default for Grid {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(std::array::from_fn::<_, 2, _>(|_| Default::default()))));
        Self {
            cells: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_slice(&self.cells));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Node {
    pub p: Rc<RefCell<Option<Point>>>,
}

impl Node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.p.clone();
        Self {
            p: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.p.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct Config {
    pub name: Rc<RefCell<Option<String>>>,
    pub port: Rc<RefCell<Option<i64>>>,
}

impl Config {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.port.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            port: __go_clone_1_0,
        }
    }
}


impl Default for Config {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            name: __go_default_0_0,
            port: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.port.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Location {
    pub p: Rc<RefCell<Option<Point>>>,
}

impl Location {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.p.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            p: __go_clone_0_0,
        }
    }
}


impl Default for Location {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Point::default())));
        Self {
            p: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.p.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct User {
    pub l: Rc<RefCell<Option<Location>>>,
}

impl User {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.l.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            l: __go_clone_0_0,
        }
    }
}


impl Default for User {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Location::default())));
        Self {
            l: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.l.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Config {
    /// A value receiver works on a copy of the struct.
    pub fn with_name(&self, name: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<Config>>> {
        let mut __self = self.__go_value_clone();
        { let new_val = name.borrow().as_ref().unwrap().clone(); *__self.name.borrow_mut() = Some(new_val); };
        Rc::new(RefCell::new(Some(__self.clone())))
    }

    pub fn bump(&self) {
        let mut __self = self.__go_value_clone();
        { let __target = __self.port.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

impl User {
    pub fn location(&self) -> Rc<RefCell<Option<Location>>> {
        return Rc::new(RefCell::new(Some({ let __selector_holder = self.l.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = __selector_guard.as_ref().unwrap().__go_value_clone(); drop(__selector_guard); __cloned })));
    }
}

pub fn r#move(p: Rc<RefCell<Option<Point>>>) -> Rc<RefCell<Option<Point>>> {
    { let __target = (*p.borrow().as_ref().unwrap()).x.clone(); let __rhs = 10; let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    { let __owned = p.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) }
}

pub fn stretch(l: Rc<RefCell<Option<Line>>>) {
    { let new_val = 100; *(*(*l.borrow().as_ref().unwrap()).b.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
}

/// A struct is a value. Each of these makes a copy, and changing the copy
/// leaves the original alone.
fn main() {
    let mut a = Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(2))), ..Default::default() })));
    let mut b = { let __owned = a.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
    { let new_val = 9; *(*b.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "assign".to_string()), format!("{}", (*a.borrow().as_ref().unwrap())), format!("{}", (*b.borrow().as_ref().unwrap())));

    let mut c: Rc<RefCell<Option<Point>>> = Rc::new(RefCell::new(Some(Default::default())));
    { let new_val = a.borrow().as_ref().unwrap().__go_value_clone(); *c.borrow_mut() = Some(new_val); };
    { let new_val = 7; *(*c.borrow().as_ref().unwrap()).y.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "assign to declared".to_string()), format!("{}", (*a.borrow().as_ref().unwrap())), format!("{}", (*c.borrow().as_ref().unwrap())));

    let mut d = r#move(Rc::new(RefCell::new(Some({ let __arg_holder = a.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))));
    println!("{} {} {}", format!("{}", "parameter and result".to_string()), format!("{}", (*a.borrow().as_ref().unwrap())), format!("{}", (*d.borrow().as_ref().unwrap())));

    let mut l1 = Rc::new(RefCell::new(Some(Line { a: Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(1))), ..Default::default() }))), b: Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(2))), y: Rc::new(RefCell::new(Some(2))), ..Default::default() }))), ..Default::default() })));
    let mut l2 = { let __owned = l1.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
    { let new_val = 9; *(*(*l2.borrow().as_ref().unwrap()).a.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "struct in struct".to_string()), format!("{}", (*l1.borrow().as_ref().unwrap())), format!("{}", (*l2.borrow().as_ref().unwrap())));

    stretch(Rc::new(RefCell::new(Some({ let __arg_holder = l1.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))));
    println!("{} {}", format!("{}", "struct in struct, parameter".to_string()), format!("{}", (*l1.borrow().as_ref().unwrap())));

    let mut points = Rc::new(RefCell::new(Some(vec![Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(1))), ..Default::default() }, Point { x: Rc::new(RefCell::new(Some(2))), y: Rc::new(RefCell::new(Some(2))), ..Default::default() }])));
    let mut e = Rc::new(RefCell::new(Some((*points.borrow().as_ref().unwrap())[(0) as usize].clone().__go_value_clone())));
    { let new_val = 50; *(*e.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "slice element".to_string()), format!("{}", format_slice(&points)), format!("{}", (*e.borrow().as_ref().unwrap())));

    { let __range_holder = points.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
        let p = &p.__go_value_clone();
        { let new_val = 60; *p.x.borrow_mut() = Some(new_val); };
    } }
    println!("{} {}", format!("{}", "range value".to_string()), format!("{}", format_slice(&points)));

    let mut arr = Rc::new(RefCell::new(Some([Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(1))), ..Default::default() }, Point { x: Rc::new(RefCell::new(Some(2))), y: Rc::new(RefCell::new(Some(2))), ..Default::default() }])));
    let mut arr2 = { let __owned = arr.borrow().as_ref().unwrap().each_ref().map(|__go_elem| __go_elem.__go_value_clone()); Rc::new(RefCell::new(Some(__owned))) };
    { let new_val = 7; *(*arr2.borrow().as_ref().unwrap())[(0) as usize].clone().x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "array".to_string()), format!("{}", format_slice(&arr)), format!("{}", format_slice(&arr2)));

    let mut g1 = Rc::new(RefCell::new(Some(Grid { cells: Rc::new(RefCell::new(Some(std::array::from_fn::<_, 2, _>(|_| Default::default())))) })));
    let mut g2 = { let __owned = g1.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
    { let new_val = 5; *(*(*g2.borrow().as_ref().unwrap()).cells.borrow().as_ref().unwrap())[(0) as usize].clone().x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "array in struct".to_string()), format!("{}", (*g1.borrow().as_ref().unwrap())), format!("{}", (*g2.borrow().as_ref().unwrap())));

    let mut base = Rc::new(RefCell::new(Some(Config { name: Rc::new(RefCell::new(Some("base".to_string()))), port: Rc::new(RefCell::new(Some(80))), ..Default::default() })));
    let mut dev = (*base.borrow().as_ref().unwrap()).with_name(Rc::new(RefCell::new(Some("dev".to_string()))));
    println!("{} {} {}", format!("{}", "value receiver".to_string()), format!("{}", (*base.borrow().as_ref().unwrap())), format!("{}", (*dev.borrow().as_ref().unwrap())));
    (*base.borrow().as_ref().unwrap()).bump();
    println!("{} {}", format!("{}", "value receiver, no result".to_string()), format!("{}", (*base.borrow().as_ref().unwrap())));

    let mut u = Rc::new(RefCell::new(Some(User { l: Rc::new(RefCell::new(Some(Location { p: Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(1))), ..Default::default() }))), ..Default::default() }))), ..Default::default() })));
    let mut v = (*u.borrow().as_ref().unwrap()).location();
    { let new_val = 9; *(*(*v.borrow().as_ref().unwrap()).p.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "getter".to_string()), format!("{}", (*(*u.borrow().as_ref().unwrap()).l.borrow().as_ref().unwrap()).clone()), format!("{}", (*v.borrow().as_ref().unwrap())));

        // A pointer inside the struct is copied as a pointer: both structs
        // point at the same Point until one of them gets a new pointer.
    let mut n1 = Rc::new(RefCell::new(Some(Node { p: Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(1))), ..Default::default() }))).clone(), ..Default::default() })));
    let mut n2 = { let __owned = n1.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
    { let new_val = 3; *(*(*n2.borrow().as_ref().unwrap()).p.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "pointer field shared".to_string()), format!("{}", { let __v = (*(*n1.borrow().as_ref().unwrap()).p.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*(*n2.borrow().as_ref().unwrap()).p.borrow().as_ref().unwrap()).clone(); __v }));
    { let new_val = Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(8))), y: Rc::new(RefCell::new(Some(8))), ..Default::default() }))).clone(); (*n2.borrow_mut().as_mut().unwrap()).p = new_val; };
    println!("{} {} {}", format!("{}", "pointer field replaced".to_string()), format!("{}", { let __v = (*(*n1.borrow().as_ref().unwrap()).p.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*(*n2.borrow().as_ref().unwrap()).p.borrow().as_ref().unwrap()).clone(); __v }));

        // A range over a slice reads each element when its turn comes, so it
        // sees a field that the loop itself wrote. The loop variable is still a
        // copy: writing to it does not change the slice.
    let mut seen = Rc::new(RefCell::new(Some(vec![Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(1))), ..Default::default() }, Point { x: Rc::new(RefCell::new(Some(2))), y: Rc::new(RefCell::new(Some(2))), ..Default::default() }])));
    { let __range_holder = seen.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
        let p = &p.__go_value_clone();
        { let new_val = 9; *(*seen.borrow().as_ref().unwrap())[(1) as usize].clone().x.borrow_mut() = Some(new_val); };
        println!("{} {}", format!("{}", "range over a slice, field written in the loop".to_string()), format!("{}", (*p.x.borrow().as_ref().unwrap())));
        { let new_val = 99; *p.x.borrow_mut() = Some(new_val); };
    } }
    println!("{} {}", format!("{}", "range over a slice, the slice afterwards".to_string()), format!("{}", format_slice(&seen)));

        // A range over an array works on a copy of the array.
    let mut fixed = Rc::new(RefCell::new(Some([Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(1))), ..Default::default() }, Point { x: Rc::new(RefCell::new(Some(2))), y: Rc::new(RefCell::new(Some(2))), ..Default::default() }])));
    { let __range_holder = fixed.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.iter().map(|__e| __e.__go_value_clone()).collect::<Vec<_>>()).unwrap_or_default(); drop(__range_guard); for p in __range_values.iter() {
        let p = &p.__go_value_clone();
        { let new_val = 9; *(*fixed.borrow().as_ref().unwrap())[(1) as usize].clone().x.borrow_mut() = Some(new_val); };
        println!("{} {}", format!("{}", "range over an array, field written in the loop".to_string()), format!("{}", (*p.x.borrow().as_ref().unwrap())));
        { let new_val = 99; *p.x.borrow_mut() = Some(new_val); };
    } }
    println!("{} {}", format!("{}", "range over an array, the array afterwards".to_string()), format!("{}", format_slice(&fixed)));
}