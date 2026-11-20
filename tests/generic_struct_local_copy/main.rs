use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub struct Box_<T: Any + 'static> {
    pub value: Rc<RefCell<Option<T>>>,
    pub count: Rc<RefCell<Option<i32>>>,
}

impl<T: Any + 'static> Box_<T> {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.value.clone();
        let __go_clone_1_0 = { let __guard = self.count.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            value: __go_clone_0_0,
            count: __go_clone_1_0,
        }
    }
}

impl<T: Any + 'static> Clone for Box_<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            count: self.count.clone(),
        }
    }
}


impl<T: Any + 'static> Default for Box_<T> {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(None));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            value: __go_default_0_0,
            count: __go_default_1_0,
        }
    }
}

impl<T: Any + 'static> std::fmt::Display for Box_<T> where T: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.count.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub fn copy_local<T: Any + 'static>(input: Rc<RefCell<Option<Box_<T>>>>) -> Rc<RefCell<Option<Box_<T>>>> {
    let mut first = { let __owned = input.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
    let mut second = { let __owned = first.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
    { let __target = (*second.borrow().as_ref().unwrap()).count.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    return { let __owned = second.borrow().as_ref().unwrap().__go_value_clone(); Rc::new(RefCell::new(Some(__owned))) };
}

fn main() {
    let mut original = Rc::new(RefCell::new(Some(Box_::<i32> { value: Rc::new(RefCell::new(Some(7))), count: Rc::new(RefCell::new(Some(10))), ..Default::default() })));
    let mut copied = copy_local::<i32>(original.clone());
    println!("{} {} {} {}", format!("{}", (*(*original.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())), format!("{}", (*(*original.borrow().as_ref().unwrap()).count.borrow().as_ref().unwrap())), format!("{}", (*(*copied.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())), format!("{}", (*(*copied.borrow().as_ref().unwrap()).count.borrow().as_ref().unwrap())));

    let mut explicit = Rc::new(RefCell::new(Some(Box_::<i32> { value: Rc::new(RefCell::new(Some(8))), count: Rc::new(RefCell::new(Some(20))), ..Default::default() })));
    let mut explicitCopy = copy_local::<i32>(explicit.clone());
    println!("{} {} {} {}", format!("{}", (*(*explicit.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())), format!("{}", (*(*explicit.borrow().as_ref().unwrap()).count.borrow().as_ref().unwrap())), format!("{}", (*(*explicitCopy.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())), format!("{}", (*(*explicitCopy.borrow().as_ref().unwrap()).count.borrow().as_ref().unwrap())));
}