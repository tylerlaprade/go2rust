use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut items = Rc::new(RefCell::new(Some(vec!["//foo".to_string(), "/*bar*/".to_string(), "//x".to_string()])));
    { let __range_holder = items.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for mut c in __range_values.iter().cloned() {
        { let _switch_val = { let __s = &(c); __s.as_bytes()[(1) as usize] };
    if _switch_val == (('/' as i32) as u8) {
            { let new_val = { let __s = &(c); let __low = (2) as usize; __s[__low..].to_string() }; c = new_val; };
            if (c.len() as i64) == (0 as i64) {
        continue
    }
            if { let __s = &(c); __s.as_bytes()[(0) as usize] } == (' ' as u8) {
        { let new_val = { let __s = &(c); let __low = (1) as usize; __s[__low..].to_string() }; c = new_val; };
    }
        } else if _switch_val == (('*' as i32) as u8) {
            { let new_val = { let __s = &(c); let __low = (2) as usize; let __high = ((c.len() as i64) - (2 as i64)) as usize; __s[__low..__high].to_string() }; c = new_val; };
        }
    }
        println!("{}", format!("{}", c));
    } }
}