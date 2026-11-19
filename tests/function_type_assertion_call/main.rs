use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

fn main() {
    let mut f: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(Box::new(move || -> (Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some("ok".to_string()))), Rc::new(RefCell::new(None)))
    }) as Box<dyn FnMut() -> (Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Rc<dyn StdError>>>>)>))).clone()) as Box<dyn Any>)));

    let (mut s, mut err) = ({
        let __f_holder = { let val = f.clone(); let guard = val.borrow(); match guard.as_ref() { Some(any_val) => any_val.downcast_ref::<Rc<RefCell<Option<Box<dyn FnMut() -> (Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Rc<dyn StdError>>>>)>>>>>().expect("type assertion failed").clone(), None => panic!("type assertion on nil interface") } };
        (|| {
            let mut __f_guard = __f_holder.borrow_mut();
            let __f: &mut Box<dyn FnMut() -> (Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Rc<dyn StdError>>>>)> = __f_guard.as_mut().expect("call of nil function");
            let __f_result = (*__f)();
            __f_result
        })()
    });
    println!("{} {}", format!("{}", { let __v = (*s.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __nil_result = (*err.borrow()).is_none(); __nil_result }));
}