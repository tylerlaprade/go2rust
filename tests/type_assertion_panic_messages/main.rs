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
    } else if let Some(v) = value.downcast_ref::<i8>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i16>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<isize>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u8>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u16>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<usize>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<Rect>() {
        format!("{}", v)
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

thread_local! {
    static __GO_RECOVER_PAYLOAD: RefCell<Option<Box<dyn Any + Send>>> = RefCell::new(None);
}

fn go_recover() -> Rc<RefCell<Option<Box<dyn Any>>>> {
    __GO_RECOVER_PAYLOAD.with(|slot| Rc::new(RefCell::new(slot.borrow_mut().take().map(|__payload| __payload as Box<dyn Any>))))
}

fn go_store_panic_payload(__payload: Box<dyn Any + Send>) {
    let __payload: Box<dyn Any + Send> = match __payload.downcast::<&'static str>() {
        Ok(__text) => Box::new(go_runtime_error_text(__text.to_string())),
        Err(__payload) => match __payload.downcast::<String>() {
            Ok(__text) => Box::new(go_runtime_error_text(*__text)),
            Err(__payload) => __payload,
        },
    };
    __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(__payload));
}

fn go_runtime_error_text(__text: String) -> String {
    if __text == "attempt to divide by zero" || __text == "attempt to calculate the remainder with a divisor of zero" {
        return "runtime error: integer divide by zero".to_string();
    }
    if let Some(__rest) = __text.strip_prefix("index out of bounds: the len is ") {
        if let Some((__len, __index)) = __rest.split_once(" but the index is ") {
            return format!("runtime error: index out of range [{}] with length {}", __index, __len);
        }
    }
    __text
}

fn go_resume_unrecovered_panic() {
    if let Some(__payload) = __GO_RECOVER_PAYLOAD.with(|slot| slot.borrow_mut().take()) {
        match __payload.downcast::<String>() {
            Ok(__text) => std::panic::panic_any(*__text),
            Err(__payload) => std::panic::resume_unwind(__payload),
        }
    }
}

fn __go_type_name(val: &dyn Any) -> &'static str {
    if val.is::<i32>() { return "int" }
    if val.is::<i64>() { return "int64" }
    if val.is::<i8>() { return "int8" }
    if val.is::<i16>() { return "int16" }
    if val.is::<u32>() { return "uint" }
    if val.is::<u64>() { return "uint64" }
    if val.is::<u8>() { return "uint8" }
    if val.is::<u16>() { return "uint16" }
    if val.is::<f64>() { return "float64" }
    if val.is::<f32>() { return "float32" }
    if val.is::<bool>() { return "bool" }
    if val.is::<String>() { return "string" }
    if val.is::<Vec<i32>>() { return "[]int" }
    if val.is::<Vec<i64>>() { return "[]int64" }
    if val.is::<Vec<f64>>() { return "[]float64" }
    if val.is::<Vec<String>>() { return "[]string" }
    if val.is::<Vec<bool>>() { return "[]bool" }
    if val.is::<Rect>() { return "main.Rect" }
    std::any::type_name_of_val(val)
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub w: Rc<RefCell<Option<i32>>>,
    pub h: Rc<RefCell<Option<i32>>>,
}

impl Rect {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.w.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.h.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            w: __go_clone_0_0,
            h: __go_clone_0_1,
        }
    }
}


impl Default for Rect {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_0_1 = Rc::new(RefCell::new(Some(0)));
        Self {
            w: __go_default_0_0,
            h: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.w.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.h.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub fn r#try(f: Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let __go_previous_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        {
        let mut r = go_recover();;
        if { let __nil_result = (*r.borrow()).is_some(); __nil_result } {
            println!("{} {}", format!("{}", "recovered:".to_string()), format!("{}", format_any(r.borrow().as_ref().unwrap().as_ref())));;
        }
    }
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        { let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = f.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };

        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
    }));
    match __go_panic_result {
        Ok(__go_value) => {
            std::panic::set_hook(__go_previous_panic_hook);
            __go_value
        }
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
                    go_store_panic_payload(__go_defer_panic);
                }
            }
            std::panic::set_hook(__go_previous_panic_hook);
            go_resume_unrecovered_panic();
            ()
        }
    }
}

fn main() {
    let mut a: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(42) as Box<dyn Any>)));
    let a_closure_clone = a.clone(); r#try(Rc::new(RefCell::new(Some(Box::new(move || {
        let _ = ({
        let val = a_closure_clone.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<String>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "string")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "string")
        }
    });
    }) as Box<dyn FnMut() -> ()>))));
    let mut b: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(42) as Box<dyn Any>)));
    let b_closure_clone = b.clone(); r#try(Rc::new(RefCell::new(Some(Box::new(move || {
        let _ = ({
        let val = b_closure_clone.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Rect>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "main.Rect")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "main.Rect")
        }
    });
    }) as Box<dyn FnMut() -> ()>))));
    let mut c: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(42) as Box<dyn Any>)));
    let c_closure_clone = c.clone(); r#try(Rc::new(RefCell::new(Some(Box::new(move || {
        let _ = ({
        let val = c_closure_clone.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Rc<RefCell<Option<Rect>>>>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "*main.Rect")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "*main.Rect")
        }
    });
    }) as Box<dyn FnMut() -> ()>))));
    let mut d: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(Rect { w: Rc::new(RefCell::new(Some(1))), h: Rc::new(RefCell::new(Some(2))), ..Default::default() }) as Box<dyn Any>)));
    let d_closure_clone = d.clone(); r#try(Rc::new(RefCell::new(Some(Box::new(move || {
        let _ = ({
        let val = d_closure_clone.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<i32>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "int")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "int")
        }
    });
    }) as Box<dyn FnMut() -> ()>))));
    let mut e: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(Rect { w: Rc::new(RefCell::new(Some(1))), h: Rc::new(RefCell::new(Some(2))), ..Default::default() }) as Box<dyn Any>)));
    let e_closure_clone = e.clone(); r#try(Rc::new(RefCell::new(Some(Box::new(move || {
        println!("{}", format!("{}", (*({
        let val = e_closure_clone.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Rect>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "main.Rect")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "main.Rect")
        }
    }).w.borrow().as_ref().unwrap())));
    }) as Box<dyn FnMut() -> ()>))));
    let mut f: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(None));
    let f_closure_clone = f.clone(); r#try(Rc::new(RefCell::new(Some(Box::new(move || {
        let _ = ({
        let val = f_closure_clone.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<i32>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "int")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "int")
        }
    });
    }) as Box<dyn FnMut() -> ()>))));
    let mut items = Rc::new(RefCell::new(Some(vec![Box::new("a".to_string()) as Box<dyn Any>, Box::new(2.5) as Box<dyn Any>])));
    let items_closure_clone = items.clone(); r#try(Rc::new(RefCell::new(Some(Box::new(move || {
        let _ = ({
        let __idx = (1) as usize;
        let __seq_holder = items_closure_clone.clone();
        let __seq_guard = __seq_holder.borrow();
        let any_val = __seq_guard.as_ref().expect("nil []any in type assertion")[__idx].as_ref();
        any_val.downcast_ref::<String>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(any_val), "string")).clone()
    });
    }) as Box<dyn FnMut() -> ()>))));
    let mut g: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new("text".to_string()) as Box<dyn Any>)));
    let g_closure_clone = g.clone(); r#try(Rc::new(RefCell::new(Some(Box::new(move || {
        let _ = ({
        let val = g_closure_clone.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            { let __handle = any_val.downcast_ref::<Rc<RefCell<Option<Vec<i32>>>>>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "[]int")).clone(); let __value = (*__handle.borrow()).clone().unwrap_or_default(); __value }
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "[]int")
        }
    });
    }) as Box<dyn FnMut() -> ()>))));
    let mut h: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(3.5) as Box<dyn Any>)));
    let h_closure_clone = h.clone(); r#try(Rc::new(RefCell::new(Some(Box::new(move || {
        println!("{}", format!("{}", ({
        let val = h_closure_clone.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<f64>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "float64")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "float64")
        }
    })));
    }) as Box<dyn FnMut() -> ()>))));
}