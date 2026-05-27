use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub(crate) const OP_TYPE: i32 = ('.' as i32);
pub(crate) const OP_ELEM: i32 = ('E' as i32);


#[derive(Debug, Clone, Default)]
pub struct opAppender {
}

impl opAppender {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for opAppender {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl opAppender {
    pub fn append_op(&self, path: Rc<RefCell<Option<Vec<u8>>>>, op: Rc<RefCell<Option<u8>>>) -> Rc<RefCell<Option<Vec<u8>>>> {
        { let __append_target = path.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push((*op.borrow().as_ref().unwrap()).clone()); __append_target.clone() }
    }
}

pub fn is_digit(c: Rc<RefCell<Option<u8>>>) -> bool {
    ('0' as u8) <= (*c.borrow().as_ref().unwrap()) && (*c.borrow().as_ref().unwrap()) <= ('9' as u8)
}

pub fn starts_with_v(s: Rc<RefCell<Option<String>>>) -> bool {
    ((*s.borrow().as_ref().unwrap()).len() as i32) > (0 as i32) && { let __s = &((*s.borrow().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] } == ('v' as u8)
}

pub fn append_op(path: Rc<RefCell<Option<Vec<u8>>>>, op: Rc<RefCell<Option<u8>>>) -> Rc<RefCell<Option<Vec<u8>>>> {
    { let __append_target = path.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push((*op.borrow().as_ref().unwrap()).clone()); __append_target.clone() }
}

pub fn classify_op(op: Rc<RefCell<Option<u8>>>) -> Rc<RefCell<Option<String>>> {
    { let _switch_val = (*op.borrow().as_ref().unwrap());
    if _switch_val == (OP_TYPE as u8) {
            return Rc::new(RefCell::new(Some("type".to_string())));
        } else if _switch_val == (OP_ELEM as u8) {
            return Rc::new(RefCell::new(Some("elem".to_string())));
        } else {
            return Rc::new(RefCell::new(Some("unknown".to_string())));
        }
    }
}

fn main() {
    println!("{} {}", format!("{}", "digit 5:".to_string()), format!("{}", is_digit(Rc::new(RefCell::new(Some(('5' as u8)))))));
    println!("{} {}", format!("{}", "digit x:".to_string()), format!("{}", is_digit(Rc::new(RefCell::new(Some(('x' as u8)))))));
    println!("{} {}", format!("{}", "version v1:".to_string()), format!("{}", starts_with_v(Rc::new(RefCell::new(Some("v1.0.0".to_string()))))));
    println!("{} {}", format!("{}", "version x1:".to_string()), format!("{}", starts_with_v(Rc::new(RefCell::new(Some("x1.0.0".to_string()))))));
    let mut path = Rc::new(RefCell::new(Some(Vec::<u8>::new())));
    { let new_val = { let __append_target = path.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(OP_TYPE as u8); __append_target.clone() }; path = new_val; };
    { let new_val = append_op(path.clone(), Rc::new(RefCell::new(Some(OP_ELEM as u8)))); path = new_val; };
    { let new_val = opAppender {  }.append_op(path.clone(), Rc::new(RefCell::new(Some(OP_ELEM as u8)))); path = new_val; };
    println!("{} {} {}", format!("{}", "op type:".to_string()), format!("{}", (*path.borrow().as_ref().unwrap())[(0) as usize].clone() == OP_TYPE as u8), format!("{}", (*classify_op(Rc::new(RefCell::new(Some((*path.borrow().as_ref().unwrap())[(0) as usize].clone())))).borrow().as_ref().unwrap())));
    println!("{} {} {}", format!("{}", "op elem:".to_string()), format!("{}", (*path.borrow().as_ref().unwrap())[(1) as usize].clone() == OP_ELEM as u8), format!("{}", (*classify_op(Rc::new(RefCell::new(Some((*path.borrow().as_ref().unwrap())[(1) as usize].clone())))).borrow().as_ref().unwrap())));
    println!("{} {} {}", format!("{}", "op method:".to_string()), format!("{}", (*path.borrow().as_ref().unwrap())[(2) as usize].clone() == OP_ELEM as u8), format!("{}", (*classify_op(Rc::new(RefCell::new(Some((*path.borrow().as_ref().unwrap())[(2) as usize].clone())))).borrow().as_ref().unwrap())));
}