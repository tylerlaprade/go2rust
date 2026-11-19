use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Config {
    pub mode: Rc<RefCell<Option<String>>>,
    pub env: Rc<RefCell<Option<String>>>,
    pub build_flags: Rc<RefCell<Option<String>>>,
    pub tests: Rc<RefCell<Option<bool>>>,
}

impl Config {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.mode.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.env.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.build_flags.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.tests.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            mode: __go_clone_0_0,
            env: __go_clone_1_0,
            build_flags: __go_clone_2_0,
            tests: __go_clone_3_0,
        }
    }
}


impl Default for Config {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_2_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_3_0 = Rc::new(RefCell::new(Some(false)));
        Self {
            mode: __go_default_0_0,
            env: __go_default_1_0,
            build_flags: __go_default_2_0,
            tests: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.mode.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.env.borrow().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.build_flags.borrow().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.tests.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}


#[derive(Debug, Clone)]
pub struct Request {
    pub mode: Rc<RefCell<Option<String>>>,
    pub env: Rc<RefCell<Option<String>>>,
    pub build_flags: Rc<RefCell<Option<String>>>,
    pub tests: Rc<RefCell<Option<bool>>>,
}

impl Request {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.mode.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.env.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.build_flags.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.tests.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            mode: __go_clone_0_0,
            env: __go_clone_1_0,
            build_flags: __go_clone_2_0,
            tests: __go_clone_3_0,
        }
    }
}


impl Default for Request {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_2_0 = Rc::new(RefCell::new(Some(String::new())));
        let __go_default_3_0 = Rc::new(RefCell::new(Some(false)));
        Self {
            mode: __go_default_0_0,
            env: __go_default_1_0,
            build_flags: __go_default_2_0,
            tests: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.mode.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.env.borrow().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.build_flags.borrow().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.tests.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}


pub fn make_request(prefix: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<Config>>>) -> Rc<RefCell<Option<Request>>>>>>> {
    let prefix_closure_clone = prefix.clone(); return Rc::new(RefCell::new(Some(Box::new(move |cfg: Rc<RefCell<Option<Config>>>| -> Rc<RefCell<Option<Request>>> {
        Rc::new(RefCell::new(Some(Request { mode: Rc::new(RefCell::new(Some(format!("{}{}", (*prefix_closure_clone.borrow().as_ref().unwrap()), (*(*cfg.borrow().as_ref().unwrap()).mode.borrow().as_ref().unwrap()).clone())))), env: Rc::new(RefCell::new(Some({ let __selector_holder = (*cfg.borrow().as_ref().unwrap()).env.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), build_flags: Rc::new(RefCell::new(Some({ let __selector_holder = (*cfg.borrow().as_ref().unwrap()).build_flags.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), tests: Rc::new(RefCell::new(Some({ let __selector_holder = (*cfg.borrow().as_ref().unwrap()).tests.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() })))
    }) as Box<dyn FnMut(Rc<RefCell<Option<Config>>>) -> Rc<RefCell<Option<Request>>>>)));
}

fn main() {
    let mut build = make_request(Rc::new(RefCell::new(Some("driver:".to_string()))));
    let mut req = { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<Config>>>) -> Rc<RefCell<Option<Request>>>> = { let mut __f_guard = build.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<Config>>>) -> Rc<RefCell<Option<Request>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(Config { mode: Rc::new(RefCell::new(Some("load".to_string()))), env: Rc::new(RefCell::new(Some("GOOS=darwin".to_string()))), build_flags: Rc::new(RefCell::new(Some("-mod=vendor".to_string()))), tests: Rc::new(RefCell::new(Some(true))), ..Default::default() })))) };
    println!("{}", format!("{}", (*(*req.borrow().as_ref().unwrap()).mode.borrow().as_ref().unwrap()).clone()));
    println!("{}", format!("{}", (*(*req.borrow().as_ref().unwrap()).env.borrow().as_ref().unwrap()).clone()));
    println!("{}", format!("{}", (*(*req.borrow().as_ref().unwrap()).build_flags.borrow().as_ref().unwrap()).clone()));
    println!("{}", format!("{}", (*(*req.borrow().as_ref().unwrap()).tests.borrow().as_ref().unwrap())));
}