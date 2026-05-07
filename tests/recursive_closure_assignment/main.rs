use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut visit: Rc<RefCell<Option<Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<bool>>>>>>> = Rc::new(RefCell::new(None));
    let visit_closure_clone = visit.clone(); { let __func_lit_target = visit_closure_clone.clone(); let new_val = Box::new(move |i: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<bool>>> {
        if (*i.borrow().as_ref().unwrap()) == 0 {
        return Rc::new(RefCell::new(Some(true)));
    }
        return { let __f_guard = visit_closure_clone.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Rc::new(RefCell::new(Some((*i.borrow().as_ref().unwrap()) - 1)))) };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<bool>>>>; *__func_lit_target.borrow_mut() = Some(new_val); };

    println!("{}", (*{ let __f_guard = visit.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)(Rc::new(RefCell::new(Some(3)))) }.borrow().as_ref().unwrap()));
}