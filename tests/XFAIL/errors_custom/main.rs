use std::cell::{RefCell};
use std::error::Error;
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
struct argError {
    arg: Rc<RefCell<Option<i32>>>,
    prob: Rc<RefCell<Option<String>>>,
}

impl argError {
    pub fn error(&mut self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("{} - {}", (*self.arg.borrow().as_ref().unwrap()), (*self.prob.borrow().as_ref().unwrap())))));
    }
}

impl Error for argError {}

impl Display for argError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", (*self.error().borrow_mut().as_mut().unwrap()))
    }
}

pub fn f1(arg: Rc<RefCell<Option<i32>>>) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>>) {

    if (*arg.borrow_mut().as_mut().unwrap()) == 42 {
        return (Rc::new(RefCell::new(Some(-1))), Rc::new(RefCell::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("can't work with 42".to_string())))));
    }
    return ({
            let __tmp_x = (*arg.borrow_mut().as_mut().unwrap());
            let __tmp_y = 3;
            Rc::new(RefCell::new(Some(__tmp_x + __tmp_y)))
        }, Rc::new(RefCell::new(None)));
}

pub fn f2(arg: Rc<RefCell<Option<i32>>>) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>>) {

    if (*arg.borrow_mut().as_mut().unwrap()) == 42 {
        return (Rc::new(RefCell::new(Some(-1))), Rc::new(RefCell::new(Some(argError { ,  }))));
    }
    return ({
            let __tmp_x = (*arg.borrow_mut().as_mut().unwrap());
            let __tmp_y = 3;
            Rc::new(RefCell::new(Some(__tmp_x + __tmp_y)))
        }, Rc::new(RefCell::new(None)));
}

fn main() {
    for i in &Rc::new(RefCell::new(Some(vec![7, 42]))) {
        let (mut r, mut e) = f1(Rc::new(RefCell::new(Some(*i))));
    if (*e.borrow()).is_some() {
        println!("{} {}", "f1 failed:".to_string(), (*e.borrow_mut().as_mut().unwrap()));
    } else {
        println!("{} {}", "f1 worked:".to_string(), (*r.borrow_mut().as_mut().unwrap()));
    }
    }
    for i in &Rc::new(RefCell::new(Some(vec![7, 42]))) {
        let (mut r, mut e) = f2(Rc::new(RefCell::new(Some(*i))));
    if (*e.borrow()).is_some() {
        println!("{} {}", "f2 failed:".to_string(), (*e.borrow_mut().as_mut().unwrap()));
    } else {
        println!("{} {}", "f2 worked:".to_string(), (*r.borrow_mut().as_mut().unwrap()));
    }
    }

    let (_, mut e) = f2(Rc::new(RefCell::new(Some(42))));
    let (mut ae, mut ok) = ({
        let val = e.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<argError>>>>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), Rc::new(RefCell::new(Some(true))))
            } else {
                (Rc::new(RefCell::new(Some(Default::default()))), Rc::new(RefCell::new(Some(false))))
            }
        } else {
            (Rc::new(RefCell::new(Some(Default::default()))), Rc::new(RefCell::new(Some(false))))
        }
    });
    if (*ok.borrow_mut().as_mut().unwrap()) {
        println!("{}", (*(*ae.borrow().as_ref().unwrap()).arg.borrow().as_ref().unwrap()));
        println!("{}", (*(*ae.borrow().as_ref().unwrap()).prob.borrow().as_ref().unwrap()));
    }
}