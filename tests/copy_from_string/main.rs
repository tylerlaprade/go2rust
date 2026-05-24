use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut buf = Rc::new(RefCell::new(Some(vec![0; (5) as usize])));
    let mut n = { let _src = "hello".to_string().as_bytes().to_vec(); let _n = std::cmp::min(((*buf.borrow().as_ref().unwrap())).len(), _src.len()); for _i in 0.._n { (*buf.borrow_mut().as_mut().unwrap())[_i] = _src[_i].clone(); } Rc::new(RefCell::new(Some(_n as i32))) };
    println!("{} {}", format!("{}", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8((*buf.borrow().as_ref().unwrap()).clone()).unwrap()))).borrow().as_ref().unwrap())));

    let mut buf2 = Rc::new(RefCell::new(Some(vec![0; (3) as usize])));
    let mut n2 = { let _src = "transpile".to_string().as_bytes().to_vec(); let _n = std::cmp::min(((*buf2.borrow().as_ref().unwrap())).len(), _src.len()); for _i in 0.._n { (*buf2.borrow_mut().as_mut().unwrap())[_i] = _src[_i].clone(); } Rc::new(RefCell::new(Some(_n as i32))) };
    println!("{} {}", format!("{}", { let __v = (*n2.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8((*buf2.borrow().as_ref().unwrap()).clone()).unwrap()))).borrow().as_ref().unwrap())));

    let mut fingerprint: Rc<RefCell<Option<[u8; 3]>>> = Rc::new(RefCell::new(Some(std::array::from_fn(|_| 0))));
    let mut input = Rc::new(RefCell::new(Some("abcdef".to_string())));
    let mut n3 = { let _dst_start = 0; let _dst_len = (*fingerprint.borrow().as_ref().unwrap()).len() - _dst_start; let _src = (*Rc::new(RefCell::new(Some({ let __s = &((*input.borrow().as_ref().unwrap()).clone()); __s[(((*input.borrow().as_ref().unwrap()).len() as i32) - (3 as i32)) as usize..].to_string() }))).borrow().as_ref().unwrap()).clone().as_bytes().to_vec(); let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*fingerprint.borrow_mut().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Rc::new(RefCell::new(Some(_n as i32))) };
    println!("{} {}", format!("{}", { let __v = (*n3.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8((*Rc::new(RefCell::new(Some({ let __seq = { let __seq_holder = fingerprint.clone(); let __seq_guard = __seq_holder.borrow(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[..].to_vec() }))).borrow().as_ref().unwrap()).clone()).unwrap()))).borrow().as_ref().unwrap())));

    let mut buf3 = Rc::new(RefCell::new(Some(vec![0; (4) as usize])));
    { let _src = "xxxx".to_string().as_bytes().to_vec(); let _n = std::cmp::min(((*buf3.borrow().as_ref().unwrap())).len(), _src.len()); for _i in 0.._n { (*buf3.borrow_mut().as_mut().unwrap())[_i] = _src[_i].clone(); } Rc::new(RefCell::new(Some(_n as i32))) };
    let mut n4 = { let _dst_start = (1) as usize; let _dst_len = ((3) as usize) - _dst_start; let _src = (*Rc::new(RefCell::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.borrow(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize..(2) as usize].to_vec() }))).borrow().as_ref().unwrap()).clone(); let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*buf3.borrow_mut().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Rc::new(RefCell::new(Some(_n as i32))) };
    println!("{} {}", format!("{}", { let __v = (*n4.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8((*buf3.borrow().as_ref().unwrap()).clone()).unwrap()))).borrow().as_ref().unwrap())));
}