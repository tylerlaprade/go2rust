use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut bytes = Rc::new(RefCell::new(Some(("hi\tthere\nworld".to_string()).as_bytes().to_vec())));
    { let __range_holder = bytes.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, b) in __range_values.iter().copied().enumerate() {
        { let _switch_val = b;
    if _switch_val == (('\t' as i32) as u8) || _switch_val == (('\n' as i32) as u8) || _switch_val == (('\r' as i32) as u8) {
            (*bytes.borrow_mut().as_mut().unwrap())[(i) as usize] = (' ' as i32) as u8;
        }
    }
    } }
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8((*bytes.borrow().as_ref().unwrap()).clone()).unwrap()))).borrow().as_ref().unwrap())));
}