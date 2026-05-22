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

    fn r#do<F>(&self, mut f: F)
    where
        F: FnMut(),
    {
        let mut done = self.done.borrow_mut();
        if !*done {
            *done = true;
            drop(done);
            f();
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

    { let __once = once.clone(); __once.r#do(|| {
        { let mut guard = count.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }); };
    { let __once = once.clone(); __once.r#do(|| {
        { let mut guard = count.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 10); };
    }); };

    println!("{} {}", format!("{}", "count:".to_string()), format!("{}", { let __v = (*count.borrow().as_ref().unwrap()).clone(); __v }));
}