use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};


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
    } else {
        "<unknown>".to_string()
    }
}

#[derive(Clone, Default)]
pub struct sampleBox {
    pub value: Rc<RefCell<Option<Box<dyn Any>>>>,
}

impl sampleBox {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.value.clone();
        Self {
            value: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for sampleBox {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", match self.value.borrow().as_ref() { Some(__v) => format_any(__v.as_ref()), None => "<nil>".to_string() });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl sampleBox {
    pub fn current(&self) -> Rc<RefCell<Option<Box<dyn Any>>>> {
        self.value.clone()
    }
}

pub fn classify(b: Rc<RefCell<Option<sampleBox>>>) -> Rc<RefCell<Option<String>>> {
    {
    let _ts_subject = (*b.borrow().as_ref().unwrap()).current().clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| {
        let mut __any = __v.as_ref() as &dyn Any;
        while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any>>() {
            __any = __boxed.as_ref() as &dyn Any;
        }
        __any
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<i32>()).is_some() {
        let v = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<i32>()).unwrap().clone())));
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("int:{}", { let __v = (*v.borrow().as_ref().unwrap()).clone(); __v }))));;
    } else {
        let v = _ts_subject.clone();
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some("other".to_string())));;
    }
    }
    unreachable!()
}

fn main() {
    println!("{}", format!("{}", (*classify(Rc::new(RefCell::new(Some(sampleBox { value: Rc::new(RefCell::new(Some(Box::new(7) as Box<dyn Any>))), ..Default::default() })))).borrow().as_ref().unwrap())));
}