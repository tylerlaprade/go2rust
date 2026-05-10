use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

pub fn load() -> (Rc<RefCell<Option<Vec<i32>>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let _: Rc<RefCell<Option<Vec<i32>>>> = Rc::new(RefCell::new(Some(Default::default())));
    let mut err: Rc<RefCell<Option<Box<dyn StdError>>>> = Rc::new(RefCell::new(None));

    __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
    }) as Box<dyn Fn() -> ()>))); let __f_guard = __f_holder.borrow(); let __f = __f_guard.as_ref().unwrap(); (*__f)() };
    }));
    {
        err = Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from("missing".to_string()))));;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (Rc::new(RefCell::new(Some(Default::default()))), err)
    }
}

fn main() {
    let (_, mut err) = load();
    println!("{}", (*err.borrow()).is_some());
}