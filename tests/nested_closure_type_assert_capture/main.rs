use std::any::Any;
use std::cell::{RefCell};
use std::rc::{Rc};

pub fn run(r#fn: Rc<RefCell<Option<Box<dyn Fn() -> ()>>>>) {
    { let __f_guard = r#fn.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)() };
}

fn main() {
    let mut v: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(7) as Box<dyn Any>)));

    let v_closure_clone = v.clone(); let mut f = Rc::new(RefCell::new(Some(Box::new(move || {
        let v_closure_clone_closure_clone = v_closure_clone.clone(); run(Rc::new(RefCell::new(Some(Box::new(move || {
        let (_, mut ok) = ({
        let val = v_closure_clone_closure_clone.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<i32>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(false))))
        }
    });
        println!("{}", { let __v = (*ok.borrow().as_ref().unwrap()).clone(); __v });
    }) as Box<dyn Fn() -> ()>))));
        let (_, mut ok) = ({
        let val = v_closure_clone.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<i32>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(false))))
        }
    });
        println!("{}", { let __v = (*ok.borrow().as_ref().unwrap()).clone(); __v });
    }) as Box<dyn Fn() -> ()>)));

    { let __f_guard = f.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)() };
}