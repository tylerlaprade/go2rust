use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub const STATE_IDLE: i32 = 0;
pub const STATE_CONNECTED: i32 = 1;
pub const STATE_ERROR: i32 = 2;
pub const STATE_RETRYING: i32 = 3;


#[derive(Debug, Clone)]
pub struct ServerState(Rc<RefCell<Option<i32>>>);

impl Display for ServerState {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().borrow().as_ref().unwrap()))
    }
}


pub(crate) struct GoGlobal<T> {
    value: std::cell::UnsafeCell<Option<T>>,
}
unsafe impl<T> Sync for GoGlobal<T> {}
impl<T> GoGlobal<T> {
    pub(crate) const fn new() -> Self {
        Self { value: std::cell::UnsafeCell::new(None) }
    }
    pub(crate) fn borrow(&'static self) -> &'static Option<T> {
        unsafe { &*self.value.get() }
    }
    pub(crate) fn borrow_mut(&'static self) -> &'static mut Option<T> {
        unsafe { &mut *self.value.get() }
    }
    pub(crate) fn clone(&'static self) -> std::rc::Rc<std::cell::RefCell<Option<T>>> where T: Clone {
        std::rc::Rc::new(std::cell::RefCell::new(self.borrow().clone()))
    }
}

pub(crate) static stateName: GoGlobal<BTreeMap<i32, Rc<RefCell<Option<String>>>>> = GoGlobal::new();


pub(crate) fn __go_init_globals() {
    *stateName.borrow_mut() = Some(BTreeMap::new());
    *stateName.borrow_mut() = Some((*Rc::new(RefCell::new(Some(BTreeMap::<i32, Rc<RefCell<Option<String>>>>::from([(STATE_IDLE, Rc::new(RefCell::new(Some("idle".to_string())))), (STATE_CONNECTED, Rc::new(RefCell::new(Some("connected".to_string())))), (STATE_ERROR, Rc::new(RefCell::new(Some("error".to_string())))), (STATE_RETRYING, Rc::new(RefCell::new(Some("retrying".to_string()))))])))).borrow().as_ref().unwrap()).clone());
}


impl ServerState {
    pub fn string(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some((*stateName.borrow().as_ref().unwrap()).get(&(*self.0.borrow().as_ref().unwrap())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()))));
    }
}

fn main() {
    __go_init_all();
    let mut ns = transition(Rc::new(RefCell::new(Some(ServerState(Rc::new(RefCell::new(Some(STATE_IDLE as i32))))))));
    println!("{}", { let __v = (*ns.borrow().as_ref().unwrap()).clone(); __v });

    let mut ns2 = transition(Rc::new(RefCell::new(Some((*ns.borrow().as_ref().unwrap()).clone()))));
    println!("{}", { let __v = (*ns2.borrow().as_ref().unwrap()).clone(); __v });
}

pub fn transition(s: Rc<RefCell<Option<ServerState>>>) -> Rc<RefCell<Option<ServerState>>> {

    { let _switch_val = (*(*s.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap());
    if _switch_val == (STATE_IDLE) {
            return Rc::new(RefCell::new(Some(ServerState(Rc::new(RefCell::new(Some(STATE_CONNECTED as i32)))))));
        } else if _switch_val == (STATE_CONNECTED) || _switch_val == (STATE_RETRYING) {
            return Rc::new(RefCell::new(Some(ServerState(Rc::new(RefCell::new(Some(STATE_IDLE as i32)))))));
        } else if _switch_val == (STATE_ERROR) {
            return Rc::new(RefCell::new(Some(ServerState(Rc::new(RefCell::new(Some(STATE_ERROR as i32)))))));
        } else {
            panic!("unknown state: {}", { let __v = (*s.borrow().as_ref().unwrap()).clone(); __v });
        }
    }
}

pub(crate) fn __go_init_all() {
    __go_init_globals();
}
