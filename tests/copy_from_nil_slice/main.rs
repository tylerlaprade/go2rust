use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut dst = Rc::new(RefCell::new(Some(("abc".to_string()).as_bytes().to_vec())));
    let mut src: Rc<RefCell<Option<Vec<u8>>>> = Rc::new(RefCell::new(None));
    let mut n = {
        let _src = { let __copy_src_holder = src.clone(); let __copy_src_guard = __copy_src_holder.borrow(); __copy_src_guard.as_ref().cloned().unwrap_or_default() };
        let _n = std::cmp::min((*dst.borrow().as_ref().unwrap()).len(), _src.len());
        for _i in 0.._n {
            (*dst.borrow_mut().as_mut().unwrap())[_i] = _src[_i].clone();
        }
        Rc::new(RefCell::new(Some(_n as i32)))
    };

    println!("{}", format!("{}", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v }));
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8((*dst.borrow().as_ref().unwrap()).clone()).unwrap()))).borrow().as_ref().unwrap())));
}