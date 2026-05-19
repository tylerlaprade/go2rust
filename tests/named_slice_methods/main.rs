use std::cell::{RefCell};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Names(pub Rc<RefCell<Option<Vec<String>>>>);


impl Names {
    pub fn len(&self) -> Rc<RefCell<Option<i32>>> {
        return Rc::new(RefCell::new(Some({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.borrow(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32)));
    }

    pub fn first(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some({ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.borrow(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() })));
    }

    pub fn join(&self) -> Rc<RefCell<Option<String>>> {
        let mut out = Rc::new(RefCell::new(Some("".to_string())));
        { let __range_holder = self.0.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for (i, name) in __range_values.iter().enumerate() {
        if i > 0 {
        { (*out.borrow_mut().as_mut().unwrap()).push_str(&",".to_string()); };
    }
        { (*out.borrow_mut().as_mut().unwrap()).push_str(&name); };
    } }
        return out.clone();
    }
}

fn main() {
    let mut names = Rc::new(RefCell::new(Some(Names(Rc::new(RefCell::new(Some(vec!["ada".to_string(), "grace".to_string()])))))));
    println!("{} {}", format!("{}", "Len:".to_string()), format!("{}", (*(*names.borrow().as_ref().unwrap()).len().borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "First:".to_string()), format!("{}", (*(*names.borrow().as_ref().unwrap()).first().borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "Join:".to_string()), format!("{}", (*(*names.borrow().as_ref().unwrap()).join().borrow().as_ref().unwrap())));
}