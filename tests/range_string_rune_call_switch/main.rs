use std::cell::{RefCell};
use std::rc::{Rc};

pub fn is_upper(r: Rc<RefCell<Option<i32>>>) -> bool {
    (*r.borrow().as_ref().unwrap()) >= ('A' as i32) && (*r.borrow().as_ref().unwrap()) <= ('Z' as i32)
}

pub fn to_lower(r: Rc<RefCell<Option<i32>>>) -> u8 {
    if (*r.borrow().as_ref().unwrap()) >= ('A' as i32) && (*r.borrow().as_ref().unwrap()) <= ('Z' as i32) {
        return (*Rc::new(RefCell::new(Some(((*r.borrow().as_ref().unwrap()) + (('a' as i32) - ('A' as i32)) as i32) as u8))).borrow().as_ref().unwrap());
    }
    (*Rc::new(RefCell::new(Some((*r.borrow().as_ref().unwrap()) as u8))).borrow().as_ref().unwrap())
}

pub fn classify(r: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<String>>> {
    { let _switch_val = (*r.borrow().as_ref().unwrap());
    if _switch_val == (('\n' as i32)) {
            return Rc::new(RefCell::new(Some("newline".to_string())));
        } else if _switch_val == (('A' as i32)) {
            return Rc::new(RefCell::new(Some("upper-a".to_string())));
        } else {
            return Rc::new(RefCell::new(Some("other".to_string())));
        }
    }
}

fn main() {
    for (_, r) in "A\nz".to_string().char_indices() {
        let mut rangeClass = Rc::new(RefCell::new(Some("other".to_string())));
        { let _switch_val = r;
    if _switch_val == ('\n') {
            { let new_val = "range-newline".to_string(); *rangeClass.borrow_mut() = Some(new_val); };
        } else if _switch_val == ('A') {
            { let new_val = "range-upper-a".to_string(); *rangeClass.borrow_mut() = Some(new_val); };
        }
    }
        let mut control = Rc::new(RefCell::new(Some(false)));
        if (r as i32) < (0x20 as i32) || (r as i32) == (0x7f as i32) || (r as i32) > (0x7e as i32) {
        { let new_val = true; *control.borrow_mut() = Some(new_val); };
    }
        println!("{} {} {} {} {}", format!("{}", is_upper(Rc::new(RefCell::new(Some(r as i32))))), format!("{}", (*Rc::new(RefCell::new(Some(to_lower(Rc::new(RefCell::new(Some(r as i32)))) as i32))).borrow().as_ref().unwrap())), format!("{}", (*classify(Rc::new(RefCell::new(Some(r as i32)))).borrow().as_ref().unwrap())), format!("{}", { let __v = (*rangeClass.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*control.borrow().as_ref().unwrap()).clone(); __v }));
    }
}