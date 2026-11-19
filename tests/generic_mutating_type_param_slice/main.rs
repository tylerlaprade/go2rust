use std::any::Any;
use std::cell::{RefCell};
use std::rc::{Rc};


fn go_any_clone(value: &dyn Any) -> Box<dyn Any> {
    if let Some(v) = value.downcast_ref::<i32>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<i64>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<i8>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<i16>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u32>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u64>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u8>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<u16>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<usize>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<isize>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<f64>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<f32>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<String>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<&'static str>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<bool>() { return Box::new(*v) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<char>() { return Box::new(*v) as Box<dyn Any>; }

    panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>")
}

pub trait GoValueClone {
    fn go_value_clone(&self) -> Self;
}

macro_rules! impl_go_value_clone_copy {
    ($($t:ty),* $(,)?) => {
        $(impl GoValueClone for $t {
            fn go_value_clone(&self) -> Self { *self }
        })*
    };
}

impl_go_value_clone_copy!(bool, char, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, &'static str);

impl GoValueClone for String {
    fn go_value_clone(&self) -> Self { self.clone() }
}

impl GoValueClone for Box<dyn Any> {
    fn go_value_clone(&self) -> Self { go_any_clone(self.as_ref()) }
}
	
pub fn sort_pair<S, E: Any + GoValueClone + 'static>(x: Rc<RefCell<Option<Vec<Rc<RefCell<Option<E>>>>>>>, less: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<E>>>, Rc<RefCell<Option<E>>>) -> bool>>>>) {
    if { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<E>>>, Rc<RefCell<Option<E>>>) -> bool> = { let mut __f_guard = less.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<E>>>, Rc<RefCell<Option<E>>>) -> bool> }; let __f = unsafe { &mut *__f_ptr }; (*__f)((*x.borrow().as_ref().unwrap())[(1) as usize].clone(), (*x.borrow().as_ref().unwrap())[(0) as usize].clone()) } {
        {
            let __tmp_0 = (*x.borrow().as_ref().unwrap())[(1) as usize].clone();
            let __tmp_1 = (*x.borrow().as_ref().unwrap())[(0) as usize].clone();
            { let __idx = (0) as usize; (*x.borrow_mut().as_mut().unwrap())[__idx] = __tmp_0; }
            { let __idx = (1) as usize; (*x.borrow_mut().as_mut().unwrap())[__idx] = __tmp_1; }
        };
    }
}

pub fn less_string(a: Rc<RefCell<Option<String>>>, b: Rc<RefCell<Option<String>>>) -> bool {
    (*a.borrow().as_ref().unwrap()) < (*b.borrow().as_ref().unwrap())
}

fn main() {
    let mut values = Rc::new(RefCell::new(Some(vec!["b".to_string(), "a".to_string()])));
    { let __slice_holder_0 = values.clone(); let __slice_arg_0 = { let __slice_guard_0 = __slice_holder_0.borrow(); Rc::new(RefCell::new(__slice_guard_0.as_ref().map(|__v| __v.iter().cloned().map(|__elem| Rc::new(RefCell::new(Some(__elem)))).collect::<Vec<_>>()))) }; sort_pair::<Vec<String>, String>(__slice_arg_0.clone(), Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<String>>>, __arg1: Rc<RefCell<Option<String>>>| -> bool { less_string(__arg0, __arg1) }) as Box<dyn FnMut(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> bool>)))); let __converted_values_0 = { let __converted_guard_0 = __slice_arg_0.borrow(); __converted_guard_0.as_ref().map(|__v| __v.iter().cloned().map(|__elem| (*__elem.borrow().as_ref().unwrap()).clone()).collect::<Vec<_>>()) }; *__slice_holder_0.borrow_mut() = __converted_values_0; };
    println!("{} {}", format!("{}", (*values.borrow().as_ref().unwrap())[(0) as usize].clone()), format!("{}", (*values.borrow().as_ref().unwrap())[(1) as usize].clone()));
}