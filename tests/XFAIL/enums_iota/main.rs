use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

const STATE_IDLE: i32 = 0;
const STATE_CONNECTED: i32 = 1;
const STATE_ERROR: i32 = 2;
const STATE_RETRYING: i32 = 3;


#[derive(Debug, Clone)]
struct ServerState(Rc<RefCell<Option<i32>>>);

impl Display for ServerState {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}


impl ServerState {
    pub fn string(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some((*(*stateName.borrow().as_ref().unwrap()).get(&(*self.0.borrow().as_ref().unwrap())).unwrap().borrow().as_ref().unwrap()))));
    }
}

fn main() {
    let mut ns = transition(StateIdle.clone());
    println!("{}", (*ns.borrow_mut().as_mut().unwrap()));

    let mut ns2 = transition(ns.clone());
    println!("{}", (*ns2.borrow_mut().as_mut().unwrap()));
}

pub fn transition(s: Rc<RefCell<Option<ServerState>>>) -> Rc<RefCell<Option<ServerState>>> {

    match (*s.borrow_mut().as_mut().unwrap()) {
        STATE_IDLE => {
            return StateConnected.clone();
        }
        STATE_CONNECTED | STATE_RETRYING => {
            return StateIdle.clone();
        }
        STATE_ERROR => {
            return StateError.clone();
        }
        _ => {
            panic!("unknown state: {}", (*s.borrow_mut().as_mut().unwrap()));
        }
    }
}