use std::cell::{RefCell};
use std::rc::{Rc};


#[derive(Clone, Debug)]
struct GoOnce {
    done: std::rc::Rc<std::cell::RefCell<bool>>,
}

impl GoOnce {
    fn new() -> Self {
        GoOnce {
            done: std::rc::Rc::new(std::cell::RefCell::new(false)),
        }
    }

    fn r#do(&self, f: Rc<RefCell<Option<Box<dyn Fn() -> ()>>>>) {
        let mut done = self.done.borrow_mut();
        if !*done {
            *done = true;
            drop(done);
            let guard = f.borrow();
            if let Some(callback) = guard.as_ref() {
                callback();
            }
        }
    }
}

impl Default for GoOnce {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let mut once = GoOnce::new();
    let mut count = Rc::new(RefCell::new(Some(0)));

    let count_closure_clone = count.clone(); once.r#do(Rc::new(RefCell::new(Some(Box::new(move || {
        { let mut guard = count_closure_clone.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }) as Box<dyn Fn() -> ()>))));
    let count_closure_clone = count.clone(); once.r#do(Rc::new(RefCell::new(Some(Box::new(move || {
        { let mut guard = count_closure_clone.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 10); };
    }) as Box<dyn Fn() -> ()>))));

    println!("{} {}", "count:".to_string(), { let __v = (*count.borrow().as_ref().unwrap()).clone(); __v });
}