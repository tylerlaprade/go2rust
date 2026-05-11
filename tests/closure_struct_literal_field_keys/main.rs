use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub mode: Rc<RefCell<Option<String>>>,
    pub env: Rc<RefCell<Option<String>>>,
    pub build_flags: Rc<RefCell<Option<String>>>,
    pub tests: Rc<RefCell<Option<bool>>>,
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.mode.borrow().as_ref().unwrap()), (*self.env.borrow().as_ref().unwrap()), (*self.build_flags.borrow().as_ref().unwrap()), (*self.tests.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct Request {
    pub mode: Rc<RefCell<Option<String>>>,
    pub env: Rc<RefCell<Option<String>>>,
    pub build_flags: Rc<RefCell<Option<String>>>,
    pub tests: Rc<RefCell<Option<bool>>>,
}

impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.mode.borrow().as_ref().unwrap()), (*self.env.borrow().as_ref().unwrap()), (*self.build_flags.borrow().as_ref().unwrap()), (*self.tests.borrow().as_ref().unwrap()))
    }
}


pub fn make_request(prefix: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<Config>>>) -> Rc<RefCell<Option<Request>>>>>>> {

    let prefix_closure_clone = prefix.clone(); return Rc::new(RefCell::new(Some(Box::new(move |cfg: Rc<RefCell<Option<Config>>>| -> Rc<RefCell<Option<Request>>> {
        return Rc::new(RefCell::new(Some(Request { mode: Rc::new(RefCell::new(Some(format!("{}{}", (*prefix_closure_clone.borrow().as_ref().unwrap()), (*(*cfg.borrow().as_ref().unwrap()).mode.borrow().as_ref().unwrap()))))), env: (*cfg.borrow().as_ref().unwrap()).env.clone(), build_flags: (*cfg.borrow().as_ref().unwrap()).build_flags.clone(), tests: (*cfg.borrow().as_ref().unwrap()).tests.clone(), ..Default::default() })));
    }) as Box<dyn FnMut(Rc<RefCell<Option<Config>>>) -> Rc<RefCell<Option<Request>>>>)));
}

fn main() {
    let mut build = make_request(Rc::new(RefCell::new(Some("driver:".to_string()))));
    let mut req = { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<Config>>>) -> Rc<RefCell<Option<Request>>>> = { let mut __f_guard = build.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<Config>>>) -> Rc<RefCell<Option<Request>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some(Config { mode: Rc::new(RefCell::new(Some("load".to_string()))), env: Rc::new(RefCell::new(Some("GOOS=darwin".to_string()))), build_flags: Rc::new(RefCell::new(Some("-mod=vendor".to_string()))), tests: Rc::new(RefCell::new(Some(true))), ..Default::default() })))) };
    println!("{}", (*(*req.borrow().as_ref().unwrap()).mode.borrow().as_ref().unwrap()));
    println!("{}", (*(*req.borrow().as_ref().unwrap()).env.borrow().as_ref().unwrap()));
    println!("{}", (*(*req.borrow().as_ref().unwrap()).build_flags.borrow().as_ref().unwrap()));
    println!("{}", (*(*req.borrow().as_ref().unwrap()).tests.borrow().as_ref().unwrap()));
}