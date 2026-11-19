use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};


thread_local! {
    static GO_FORMAT_ANY_DEPTH: std::cell::Cell<usize> = std::cell::Cell::new(0);
}

// Runs format one level further inside the value being printed.
fn go_format_any_nested(format: impl FnOnce() -> String) -> String {
    struct Depth;
    impl Drop for Depth {
        fn drop(&mut self) {
            GO_FORMAT_ANY_DEPTH.with(|depth| depth.set(depth.get() - 1));
        }
    }
    GO_FORMAT_ANY_DEPTH.with(|depth| depth.set(depth.get() + 1));
    let _depth = Depth;
    format()
}

fn format_any(value: &dyn Any) -> String {
    if let Some(v) = value.downcast_ref::<i32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<String>() {
        v.clone()
    } else if let Some(v) = value.downcast_ref::<&str>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<bool>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<node>() {
        format!("{}", v)
    } else {
        "<unknown>".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct node {
    pub value: Rc<RefCell<Option<i32>>>,
}

impl node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            value: __go_clone_0_0,
        }
    }
}


impl Default for node {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            value: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Clone, Default)]
pub struct holder {
    pub value: Rc<RefCell<Option<Box<dyn Any>>>>,
}

impl holder {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.value.clone();
        Self {
            value: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", match self.value.borrow().as_ref() { Some(__v) => go_format_any_nested(|| format_any(__v.as_ref())), None => "<nil>".to_string() });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl node {
    pub fn value(&self) -> i32 {
        return (*self.value.borrow().as_ref().unwrap());
    }
}

impl holder {
    pub fn current(&self) -> Rc<RefCell<Option<Box<dyn Any>>>> {
        self.value.clone()
    }
}

fn main() {
    let mut T = holder { value: Rc::new(RefCell::new(Some(Box::new(node { value: Rc::new(RefCell::new(Some(7))), ..Default::default() }) as Box<dyn Any>))), ..Default::default() }.current();
    let T_closure_clone = T.clone(); let mut visit = Rc::new(RefCell::new(Some(Box::new(move || -> i32 {
        {
    let _ts_subject = T_closure_clone.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| {
        let mut __any = __v.as_ref() as &dyn Any;
        while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any>>() {
            __any = __boxed.as_ref() as &dyn Any;
        }
        __any
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<node>()).is_some() {
        let T = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<node>()).unwrap().clone())));
        drop(_ts_guard);
        return (*T.borrow().as_ref().unwrap()).value();;
    } else {
        let T = _ts_subject.clone();
        drop(_ts_guard);
        return 0;;
    }
    }
    unreachable!()
    }) as Box<dyn FnMut() -> i32>)));
    println!("{}", format!("{}", { let __f_ptr: *mut Box<dyn FnMut() -> i32> = { let mut __f_guard = visit.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> i32> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }));
}