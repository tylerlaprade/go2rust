use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

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


/// Status has a String method on the value, so *Status has it too.
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


/// Gauge has a String method on the pointer only.
#[derive(Debug, Clone)]
pub struct Gauge {
    pub level: Rc<RefCell<Option<i32>>>,
}

impl Gauge {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.level.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            level: __go_clone_0_0,
        }
    }
}


impl Default for Gauge {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            level: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Gauge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().borrow().as_ref().unwrap()))
    }
}


/// Broken is an error through its pointer.
#[derive(Debug, Clone)]
pub struct Broken {
    pub why: Rc<RefCell<Option<String>>>,
}

impl Broken {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.why.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            why: __go_clone_0_0,
        }
    }
}


impl Default for Broken {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            why: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Broken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


/// Timeout is an error through its value.
#[derive(Debug, Clone)]
pub struct Timeout {
    pub seconds: Rc<RefCell<Option<i32>>>,
}

impl Timeout {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.seconds.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            seconds: __go_clone_0_0,
        }
    }
}


impl Default for Timeout {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            seconds: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Timeout {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


impl Status {
    pub fn string(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("status {}", (*self.code.borrow().as_ref().unwrap())))))
    }
}

impl Gauge {
    pub fn string(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("gauge {}", (*self.level.borrow().as_ref().unwrap())))))
    }
}

impl Broken {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("{}{}", "broken: ".to_string(), (*self.why.clone().borrow().as_ref().unwrap())))))
    }
}

impl StdError for Broken {}


impl Timeout {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("timeout after {}s", (*self.seconds.borrow().as_ref().unwrap())))))
    }
}

impl StdError for Timeout {}


pub fn find(ok: Rc<RefCell<Option<bool>>>) -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
    if (*ok.borrow().as_ref().unwrap()) {
        return Rc::new(RefCell::new(None));
    }
    Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("missing".to_string())))))
}

pub fn lookup(ok: Rc<RefCell<Option<bool>>>) -> Rc<RefCell<Option<Point>>> {
    if (*ok.borrow().as_ref().unwrap()) {
        return Rc::new(RefCell::new(Some(Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(2))), ..Default::default() })));
    }
    Rc::new(RefCell::new(None))
}

fn main() {
        // A nil error prints as <nil>, whichever print call it reaches and
        // whether it comes from a variable or straight from a call.
    let mut err = find(Rc::new(RefCell::new(Some(true))));
    println!("{}", format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{} {}", format!("{}", "result:".to_string()), format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    print!("{}\n", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
    print!("{} and {}\n", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }, 3);
    println!("{}", format!("{}", { let __text = match find(Rc::new(RefCell::new(Some(true)))).borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text })))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(format!("got {}", { let __text = match find(Rc::new(RefCell::new(Some(true)))).borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text })))).borrow().as_ref().unwrap())));

        // An error that is not nil prints its message, as before.
    { let __rhs_holder = find(Rc::new(RefCell::new(Some(false)))).clone(); let new_val = (*__rhs_holder.borrow()).clone(); *err.borrow_mut() = new_val; };
    println!("{}", format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{} {}", format!("{}", "result:".to_string()), format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    print!("{} {}\n", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }, { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
    println!("{}", format!("{}", { let __text = match find(Rc::new(RefCell::new(Some(false)))).borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

        // A nil pointer prints as <nil>.
    let mut p: Rc<RefCell<Option<Point>>> = Rc::new(RefCell::new(None));
    println!("{}", format!("{}", { let __text = match p.borrow().as_ref() { Some(__value) => format!("&{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{} {}", format!("{}", "point:".to_string()), format!("{}", { let __text = match p.borrow().as_ref() { Some(__value) => format!("&{}", __value), None => "<nil>".to_string() }; __text }));
    print!("{}\n", { let __text = match p.borrow().as_ref() { Some(__value) => format!("&{}", __value), None => "<nil>".to_string() }; __text });
    println!("{}", format!("{}", { let __text = match lookup(Rc::new(RefCell::new(Some(false)))).borrow().as_ref() { Some(__value) => format!("&{}", __value), None => "<nil>".to_string() }; __text }));
    { let new_val = lookup(Rc::new(RefCell::new(Some(true)))).clone(); p = new_val; };
    println!("{}", format!("{}", { let __text = match p.borrow().as_ref() { Some(__value) => format!("&{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{}", format!("{}", { let __text = match lookup(Rc::new(RefCell::new(Some(true)))).borrow().as_ref() { Some(__value) => format!("&{}", __value), None => "<nil>".to_string() }; __text }));

        // A pointer whose type has a String or Error method prints through the
        // method, with no & in front. A nil one prints as <nil>.
    let mut s = Rc::new(RefCell::new(Some(Status { code: Rc::new(RefCell::new(Some(7))), ..Default::default() })));
    println!("{}", format!("{}", { let __text = match s.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    print!("{} {}\n", { let __text = match s.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }, { let __text = match s.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text });
    let mut g = Rc::new(RefCell::new(Some(Gauge { level: Rc::new(RefCell::new(Some(3))), ..Default::default() })));
    println!("{}", format!("{}", { let __text = match g.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", { let __text = match g.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text })))).borrow().as_ref().unwrap())));
    let mut b = Rc::new(RefCell::new(Some(Broken { why: Rc::new(RefCell::new(Some("z".to_string()))), ..Default::default() })));
    println!("{}", format!("{}", { let __text = match b.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{} {} {} {}", format!("{}", "gauge:".to_string()), format!("{}", { let __text = match g.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", "broken:".to_string()), format!("{}", { let __text = match b.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    let mut none: Rc<RefCell<Option<Status>>> = Rc::new(RefCell::new(None));
    println!("{}", format!("{}", { let __text = match none.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    let mut empty: Rc<RefCell<Option<Gauge>>> = Rc::new(RefCell::new(None));
    println!("{}", format!("{}", { let __text = match empty.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

        // A struct or a pointer that implements error can start an error
        // variable, and prints as the error it is.
    let mut declared: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::new(Broken { why: Rc::new(RefCell::new(Some("declared".to_string()))), ..Default::default() }) as Rc<dyn StdError>)));
    println!("{}", format!("{}", { let __text = match declared.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    let mut plain: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::new(Timeout { seconds: Rc::new(RefCell::new(Some(3))), ..Default::default() }) as Rc<dyn StdError>)));
    println!("{}", format!("{}", { let __text = match plain.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{} {}", format!("{}", { let __nil_result = (*declared.borrow()).is_some(); __nil_result }), format!("{}", { let __nil_result = (*plain.borrow()).is_some(); __nil_result }));
    let mut unset: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(None));
    println!("{} {}", format!("{}", { let __text = match unset.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __nil_result = (*unset.borrow()).is_none(); __nil_result }));
}