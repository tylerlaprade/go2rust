use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut data = Rc::new(RefCell::new(Some("Hello, World!".to_string())));
    let mut encoded = (*base64::std_encoding.borrow().as_mut().unwrap()).encode_to_string(Rc::new(RefCell::new(Some(Rc::new(RefCell::new(Some((*data.borrow().as_ref().unwrap()).as_bytes().to_vec())))))));
    println!("{} {}", "Encoded:".to_string(), (*encoded.borrow().as_ref().unwrap()));

    let (mut decoded, _) = (*base64::std_encoding.borrow().as_mut().unwrap()).decode_string(Rc::new(RefCell::new(Some((*encoded.borrow().as_ref().unwrap())))));
    println!("{} {}", "Decoded:".to_string(), (*Rc::new(RefCell::new(Some(String::from_utf8((*decoded.borrow().as_ref().unwrap()).clone()).unwrap()))).borrow().as_ref().unwrap()));
}