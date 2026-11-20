use std::cell::{RefCell};
use std::error::Error as StdError;
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
        let __go_plus = f.alternate();
        let __go_fmt_0 = if __go_plus { format!("{:#}", (*self.x.borrow().as_ref().unwrap())) } else { format!("{}", (*self.x.borrow().as_ref().unwrap())) };
        let __go_fmt_1 = if __go_plus { format!("{:#}", (*self.y.borrow().as_ref().unwrap())) } else { format!("{}", (*self.y.borrow().as_ref().unwrap())) };
        if __go_plus {
            return write!(f, "{{X:{} Y:{}}}", __go_fmt_0, __go_fmt_1);
        }
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Base {
    pub i_d: Rc<RefCell<Option<i64>>>,
}

impl Base {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.i_d.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            i_d: __go_clone_0_0,
        }
    }
}


impl Default for Base {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            i_d: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Base {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_plus = f.alternate();
        let __go_fmt_0 = if __go_plus { format!("{:#}", (*self.i_d.borrow().as_ref().unwrap())) } else { format!("{}", (*self.i_d.borrow().as_ref().unwrap())) };
        if __go_plus {
            return write!(f, "{{ID:{}}}", __go_fmt_0);
        }
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct Line {
    pub base: Rc<RefCell<Option<Base>>>,
    pub from: Rc<RefCell<Option<Point>>>,
    pub to: Rc<RefCell<Option<Point>>>,
    pub label: Rc<RefCell<Option<String>>>,
    pub tags: Rc<RefCell<Option<Vec<String>>>>,
    pub weight: Rc<RefCell<Option<f64>>>,
    pub ok: Rc<RefCell<Option<bool>>>,
}

impl Line {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.base.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_1_0 = { let __guard = self.from.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_1_1 = { let __guard = self.to.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_2_0 = { let __guard = self.label.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_3_0 = self.tags.clone();
        let __go_clone_4_0 = { let __guard = self.weight.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.ok.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            base: __go_clone_0_0,
            from: __go_clone_1_0,
            to: __go_clone_1_1,
            label: __go_clone_2_0,
            tags: __go_clone_3_0,
            weight: __go_clone_4_0,
            ok: __go_clone_5_0,
        }
    }
}


impl Default for Line {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Base::default())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(Point::default())));
        let __go_default_1_1 = Rc::new(RefCell::new(Some(Point::default())));
        let __go_default_2_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_3_0 = Rc::new(RefCell::new(None));
        let __go_default_4_0 = Rc::new(RefCell::new(Some(0.0)));
        let __go_default_5_0 = Rc::new(RefCell::new(Some(false)));
        Self {
            base: __go_default_0_0,
            from: __go_default_1_0,
            to: __go_default_1_1,
            label: __go_default_2_0,
            tags: __go_default_3_0,
            weight: __go_default_4_0,
            ok: __go_default_5_0,
        }
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_plus = f.alternate();
        let __go_fmt_0 = if __go_plus { format!("{:#}", (*self.base.borrow().as_ref().unwrap())) } else { format!("{}", (*self.base.borrow().as_ref().unwrap())) };
        let __go_fmt_1 = if __go_plus { format!("{:#}", (*self.from.borrow().as_ref().unwrap())) } else { format!("{}", (*self.from.borrow().as_ref().unwrap())) };
        let __go_fmt_2 = if __go_plus { format!("{:#}", (*self.to.borrow().as_ref().unwrap())) } else { format!("{}", (*self.to.borrow().as_ref().unwrap())) };
        let __go_fmt_3 = if __go_plus { format!("{:#}", (*self.label.borrow().as_ref().unwrap())) } else { format!("{}", (*self.label.borrow().as_ref().unwrap())) };
        let __go_fmt_4 = format!("{}", format_slice(&self.tags));
        let __go_fmt_5 = if __go_plus { format!("{:#}", (*self.weight.borrow().as_ref().unwrap())) } else { format!("{}", (*self.weight.borrow().as_ref().unwrap())) };
        let __go_fmt_6 = if __go_plus { format!("{:#}", (*self.ok.borrow().as_ref().unwrap())) } else { format!("{}", (*self.ok.borrow().as_ref().unwrap())) };
        if __go_plus {
            return write!(f, "{{Base:{} From:{} To:{} Label:{} Tags:{} Weight:{} Ok:{}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6);
        }
        write!(f, "{{{} {} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6)
    }
}


#[derive(Debug, Clone)]
pub struct Named {
    pub n: Rc<RefCell<Option<i64>>>,
}

impl Named {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            n: __go_clone_0_0,
        }
    }
}


impl Default for Named {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            n: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Named {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct Config {
    pub name: Rc<RefCell<Option<String>>>,
    pub retries: Rc<RefCell<Option<i64>>>,
    pub verbose: Rc<RefCell<Option<bool>>>,
}

impl Config {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.retries.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.verbose.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            retries: __go_clone_1_0,
            verbose: __go_clone_2_0,
        }
    }
}


impl Default for Config {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_2_0 = Rc::new(RefCell::new(Some(false)));
        Self {
            name: __go_default_0_0,
            retries: __go_default_1_0,
            verbose: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_plus = f.alternate();
        let __go_fmt_0 = if __go_plus { format!("{:#}", (*self.name.borrow().as_ref().unwrap())) } else { format!("{}", (*self.name.borrow().as_ref().unwrap())) };
        let __go_fmt_1 = if __go_plus { format!("{:#}", (*self.retries.borrow().as_ref().unwrap())) } else { format!("{}", (*self.retries.borrow().as_ref().unwrap())) };
        let __go_fmt_2 = if __go_plus { format!("{:#}", (*self.verbose.borrow().as_ref().unwrap())) } else { format!("{}", (*self.verbose.borrow().as_ref().unwrap())) };
        if __go_plus {
            return write!(f, "{{Name:{} Retries:{} Verbose:{}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2);
        }
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


impl Named {
    pub fn string(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("N{}", (*self.n.borrow().as_ref().unwrap())))))
    }
}

impl Line {
}

fn main() {
    let mut p = Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(2))), ..Default::default() })));
    let mut l = Rc::new(RefCell::new(Some(Line { base: Rc::new(RefCell::new(Some(Base { i_d: Rc::new(RefCell::new(Some(7))), ..Default::default() }))), from: Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(0))), y: Rc::new(RefCell::new(Some(0))), ..Default::default() }))), to: Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(3))), y: Rc::new(RefCell::new(Some(4))), ..Default::default() }))), label: Rc::new(RefCell::new(Some("diag".to_string()))), tags: Rc::new(RefCell::new(Some(vec!["a".to_string(), "b".to_string()]))), weight: Rc::new(RefCell::new(Some(2.5))), ok: Rc::new(RefCell::new(Some(true))), ..Default::default() })));
    print!("{:#}\n", (*p.borrow().as_ref().unwrap()));
    print!("{:#}\n", (*l.borrow().as_ref().unwrap()));
    print!("{}\n", (*l.borrow().as_ref().unwrap()));
    let mut s = Rc::new(RefCell::new(Some(format!("p={:#} n={:#} i={:#} d={:+} str={:#}", (*p.borrow().as_ref().unwrap()), Named { n: Rc::new(RefCell::new(Some(3))), ..Default::default() }, 5, 6, "x".to_string()))));
    println!("{}", format!("{}", { let __v = (*s.borrow().as_ref().unwrap()).clone(); __v }));
    print!("{:#}\n", { let __text = match Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("boom".to_string()))))).borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
    let mut pts = Rc::new(RefCell::new(Some(vec![Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(2))), ..Default::default() }, Point { x: Rc::new(RefCell::new(Some(3))), y: Rc::new(RefCell::new(Some(4))), ..Default::default() }])));
    print!("{:#} {}\n", { let __plus_holder = pts.clone(); let __plus_guard = __plus_holder.borrow(); match __plus_guard.as_ref() { Some(__s) => format!("[{}]", __s.iter().map(|__v| format!("{:#}", __v)).collect::<Vec<_>>().join(" ")), None => "[]".to_string() } }, format_slice(&pts));
    let mut pp = p.clone();
    print!("{:#} {}\n", { let __text = match pp.borrow().as_ref() { Some(__value) => format!("&{:#}", __value), None => "<nil>".to_string() }; __text }, { let __text = match pp.borrow().as_ref() { Some(__value) => format!("&{}", __value), None => "<nil>".to_string() }; __text });
    print!("{:#}\n", AnonymousStruct1 { a: Rc::new(RefCell::new(Some(1))), b: Rc::new(RefCell::new(Some(2))) });
    let mut cfg = Rc::new(RefCell::new(Some(Config { name: Rc::new(RefCell::new(Some("svc".to_string()))), retries: Rc::new(RefCell::new(Some(3))), ..Default::default() })));
    print!("config: {:#}\n", (*cfg.borrow().as_ref().unwrap()));
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(format!("{}|{:#}", (*cfg.borrow().as_ref().unwrap()), (*cfg.borrow().as_ref().unwrap()))))).borrow().as_ref().unwrap())));
}

#[derive(Debug, Clone)]
pub struct AnonymousStruct1 {
    pub a: Rc<RefCell<Option<i64>>>,
    pub b: Rc<RefCell<Option<i64>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.a.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.b.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            a: __go_clone_0_0,
            b: __go_clone_0_1,
        }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_0_1 = Rc::new(RefCell::new(Some(0)));
        Self {
            a: __go_default_0_0,
            b: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_plus = f.alternate();
        let __go_fmt_0 = if __go_plus { format!("{:#}", (*self.a.borrow().as_ref().unwrap())) } else { format!("{}", (*self.a.borrow().as_ref().unwrap())) };
        let __go_fmt_1 = if __go_plus { format!("{:#}", (*self.b.borrow().as_ref().unwrap())) } else { format!("{}", (*self.b.borrow().as_ref().unwrap())) };
        if __go_plus {
            return write!(f, "{{A:{} B:{}}}", __go_fmt_0, __go_fmt_1);
        }
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}
