use std::cell::{RefCell};
use std::rc::{Rc};

pub fn sort_pair__slice_string__string(x: Rc<RefCell<Option<Vec<String>>>>, less: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> bool>>>>) {
    if { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> bool> = { let mut __f_guard = less.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> bool> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some((*x.borrow().as_ref().unwrap())[(1) as usize].clone()))), Rc::new(RefCell::new(Some((*x.borrow().as_ref().unwrap())[(0) as usize].clone())))) } {
        {
            let __go_lhs_sequence_0 = x.clone();
            let __go_lhs_index_0 = (0) as usize;
            let __go_lhs_sequence_1 = x.clone();
            let __go_lhs_index_1 = (1) as usize;
            let __tmp_0 = (*x.borrow().as_ref().unwrap())[(1) as usize].clone();
            let __tmp_1 = (*x.borrow().as_ref().unwrap())[(0) as usize].clone();
            (*__go_lhs_sequence_0.borrow_mut().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.borrow_mut().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
    }
}

pub fn less_string(a: Rc<RefCell<Option<String>>>, b: Rc<RefCell<Option<String>>>) -> bool {
    (*a.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap())
}

fn main() {
    let mut values = Rc::new(RefCell::new(Some(vec!["b".to_string(), "a".to_string()])));
    sort_pair__slice_string__string(values.clone(), Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<String>>>, __arg1: Rc<RefCell<Option<String>>>| -> bool { less_string(__arg0, __arg1) }) as Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> bool>))));
    println!("{} {}", format!("{}", (*values.borrow().as_ref().unwrap())[(0) as usize].clone()), format!("{}", (*values.borrow().as_ref().unwrap())[(1) as usize].clone()));
}