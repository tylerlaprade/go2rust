use std::cell::{RefCell};
use std::rc::{Rc};

pub fn consumed_all(values: Rc<RefCell<Option<Vec<i32>>>>) -> Rc<RefCell<Option<bool>>> {

    let mut i: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(0)));
    { let new_val = 0; *i.borrow_mut() = Some(new_val); };
    while ((*i.borrow().as_ref().unwrap()) as i32) < ((*values.borrow().as_ref().unwrap()).len() as i32) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return {
            let __tmp_x = ((*i.borrow().as_ref().unwrap()) as i32);
            let __tmp_y = ((*values.borrow().as_ref().unwrap()).len() as i32);
            Rc::new(RefCell::new(Some(__tmp_x == __tmp_y)))
        };
}

pub fn last_index(values: Rc<RefCell<Option<Vec<i32>>>>) -> Rc<RefCell<Option<i32>>> {

    return Rc::new(RefCell::new(Some(((*values.borrow().as_ref().unwrap()).len() as i32) - (1 as i32))));
}

fn main() {
    let mut values = Rc::new(RefCell::new(Some(vec![1, 2, 3])));
    println!("{}", (*consumed_all(values.clone()).borrow().as_ref().unwrap()));
    println!("{}", (*last_index(values.clone()).borrow().as_ref().unwrap()));
}