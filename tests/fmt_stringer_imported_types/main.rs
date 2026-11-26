#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! print {
    ($($__arg:tt)*) => {{
        std::print!($($__arg)*);
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }};
}

fn go_any_try_clone(value: &dyn Any) -> Option<Box<dyn Any>> {
    if let Some(v) = value.downcast_ref::<i32>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<i64>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<i8>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<i16>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<u32>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<u64>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<u8>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<u16>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<usize>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<isize>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<f64>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<f32>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<&'static str>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<bool>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<char>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<String>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Celsius>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Celsius>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<example_com_fmt_stringer_imported_types_model::r#mod::Label>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<example_com_fmt_stringer_imported_types_model::r#mod::Label>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    go_clone_registered_any(value)
}

fn go_any_clone(value: &dyn Any) -> Box<dyn Any> {
    go_any_try_clone(value).unwrap_or_else(|| panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>"))
}

fn go_install_panic_exit() {
    // Go lets a program call main again; the hook goes in once, so a
    // later call keeps the hook that go_silence_panic_hook wrapped around it.
    static __GO_PANIC_EXIT: std::sync::Once = std::sync::Once::new();
    __GO_PANIC_EXIT.call_once(|| {
        std::panic::set_hook(Box::new(|__info| {
            let __text = go_panic_payload_text(__info.payload());
            {
                use std::io::Write;
                std::io::stdout().flush().ok();
            }
            eprintln!("panic: {}", __text);
            std::process::exit(2);
        }));
    });
}

fn go_panic_payload_text(__payload: &dyn Any) -> String {
    if let Some(__text) = __payload.downcast_ref::<&'static str>() {
        return go_runtime_error_text(__text.to_string());
    }
    if let Some(__text) = __payload.downcast_ref::<String>() {
        return go_runtime_error_text_of_string(__text.clone());
    }
    String::from(GO_UNPRINTABLE_PANIC_VALUE)
}

const GO_UNPRINTABLE_PANIC_VALUE: &str = "(unprintable panic value)";

fn go_runtime_error_text(__text: String) -> String {
    if __text == "attempt to divide by zero" || __text == "attempt to calculate the remainder with a divisor of zero" {
        return "runtime error: integer divide by zero".to_string();
    }
    if let Some(__rest) = __text.strip_prefix("index out of bounds: the len is ") {
        if let Some((__len, __index)) = __rest.split_once(" but the index is ") {
            return format!("runtime error: index out of range [{}] with length {}", __index, __len);
        }
    }
    // A nil pointer's cell is empty, so reading through it unwraps None.
    if __text == "called `Option::unwrap()` on a `None` value" || __text == "nil pointer dereference" || __text == "invalid memory address or nil pointer dereference" {
        return "runtime error: invalid memory address or nil pointer dereference".to_string();
    }
    __text
}

fn go_runtime_error_text_of_string(__text: String) -> String {
    // panic(s) in Go raises a String, and so does a runtime error whose text
    // Rust formats: an index out of range, or a nil pointer read through
    // expect(). The constant runtime errors arrive as a &'static str, so a
    // Go string that reads like one keeps its text.
    if __text.starts_with("index out of bounds: ") || __text == "nil pointer dereference" || __text == "invalid memory address or nil pointer dereference" {
        return go_runtime_error_text(__text);
    }
    __text
}

fn __go_type_name(val: &dyn Any) -> &'static str {
    if val.is::<i64>() { return "int" }
    if val.is::<i32>() { return "int32" }
    if val.is::<char>() { return "int32" }
    if val.is::<i8>() { return "int8" }
    if val.is::<i16>() { return "int16" }
    if val.is::<u64>() { return "uint" }
    if val.is::<u32>() { return "uint32" }
    if val.is::<u8>() { return "uint8" }
    if val.is::<u16>() { return "uint16" }
    if val.is::<f64>() { return "float64" }
    if val.is::<f32>() { return "float32" }
    if val.is::<bool>() { return "bool" }
    if val.is::<String>() { return "string" }
    if val.is::<Vec<i64>>() { return "[]int" }
    if val.is::<Vec<i32>>() { return "[]int32" }
    if val.is::<Vec<f64>>() { return "[]float64" }
    if val.is::<Vec<String>>() { return "[]string" }
    if val.is::<Vec<bool>>() { return "[]bool" }
    if val.is::<Celsius>() { return "main.Celsius" }
    if val.is::<Rc<RefCell<Option<Celsius>>>>() { return "*main.Celsius" }
    if val.is::<Rc<RefCell<Option<example_com_fmt_stringer_imported_types_model::r#mod::Label>>>>() { return "*model.Label" }
    if val.is::<example_com_fmt_stringer_imported_types_model::r#mod::Label>() { return "model.Label" }
    if let Some(__go_name) = go_registered_any_type_name(val) { return __go_name }
    std::any::type_name_of_val(val)
}

#[derive(Debug, Clone)]
pub struct Celsius {
    pub deg: Rc<RefCell<Option<i64>>>,
}

impl Celsius {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.deg.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            deg: __go_clone_0_0,
        }
    }
}


impl Default for Celsius {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            deg: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Celsius {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().borrow().as_ref().unwrap()))
    }
}


impl Celsius {
    pub fn string(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("{}C", (*self.deg.borrow().as_ref().unwrap())))))
    }
}

pub fn make_c() -> Rc<RefCell<Option<Celsius>>> {
    Rc::new(RefCell::new(Some(Celsius { deg: Rc::new(RefCell::new(Some(3))), ..Default::default() })))
}

fn main() {
    go_install_panic_exit();
    ::example_com_fmt_stringer_imported_types_model::__go_init_all();

    let mut s: Rc<RefCell<Option<fmt_Stringer>>> = Rc::new(RefCell::new(Some(fmt_Stringer::__go_from_display((*make_c().borrow().as_ref().unwrap()).clone()))));
    println!("{}", format!("{}", (*(*s.borrow().as_ref().unwrap()).string().borrow().as_ref().unwrap())));
    let mut v: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some({ go_register_any_runtime_type::<Celsius>("main.Celsius", |__value| { let __typed = __value.downcast_ref::<Celsius>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<Celsius>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any> }, None); Box::new(Celsius { deg: Rc::new(RefCell::new(Some(2))), ..Default::default() }) as Box<dyn Any> })));
    println!("{}", format!("{}", format!("{}", ({
        let val = v.clone();
        let __stringer: Result<fmt_Stringer, String> = {
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<fmt_Stringer>() { Ok(typed_val.clone()) } else if let Some(typed_val) = any_val.downcast_ref::<Celsius>() { Ok(fmt_Stringer::__go_from_display(typed_val.clone())) } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Celsius>>>>() { Ok(fmt_Stringer::__go_from_display_cell(typed_val.clone())) } else if let Some(typed_val) = any_val.downcast_ref::<example_com_fmt_stringer_imported_types_model::r#mod::Label>() { Ok(fmt_Stringer::__go_from_display(typed_val.clone())) } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<example_com_fmt_stringer_imported_types_model::r#mod::Label>>>>() { Ok(fmt_Stringer::__go_from_display_cell(typed_val.clone())) } else { Err(format!("interface conversion: {} is not fmt.Stringer: missing method String", __go_type_name(&**any_val))) }
        } else {
            Err("interface conversion: interface is nil, not fmt.Stringer".to_string())
        }
        };
        match __stringer { Ok(__s) => __s, Err(__message) => panic!("{}", __message) }
    }))));
    print!("{}\n", __go_type_name(&*(({
        let val = v.clone();
        let __stringer: Result<fmt_Stringer, String> = {
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<fmt_Stringer>() { Ok(typed_val.clone()) } else if let Some(typed_val) = any_val.downcast_ref::<Celsius>() { Ok(fmt_Stringer::__go_from_display(typed_val.clone())) } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Celsius>>>>() { Ok(fmt_Stringer::__go_from_display_cell(typed_val.clone())) } else if let Some(typed_val) = any_val.downcast_ref::<example_com_fmt_stringer_imported_types_model::r#mod::Label>() { Ok(fmt_Stringer::__go_from_display(typed_val.clone())) } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<example_com_fmt_stringer_imported_types_model::r#mod::Label>>>>() { Ok(fmt_Stringer::__go_from_display_cell(typed_val.clone())) } else { Err(format!("interface conversion: {} is not fmt.Stringer: missing method String", __go_type_name(&**any_val))) }
        } else {
            Err("interface conversion: interface is nil, not fmt.Stringer".to_string())
        }
        };
        match __stringer { Ok(__s) => __s, Err(__message) => panic!("{}", __message) }
    })).__go_value));
    let (_, mut ok) = ({
        let val = Rc::new(RefCell::new(Some(Box::new(example_com_fmt_stringer_imported_types_model::r#mod::Label { text: Rc::new(RefCell::new(Some("x".to_string()))), ..Default::default() }) as Box<dyn Any>))).clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<fmt_Stringer>() { (Rc::new(RefCell::new(Some(typed_val.clone()))), true) } else if let Some(typed_val) = any_val.downcast_ref::<Celsius>() { (Rc::new(RefCell::new(Some(fmt_Stringer::__go_from_display(typed_val.clone())))), true) } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Celsius>>>>() { (Rc::new(RefCell::new(Some(fmt_Stringer::__go_from_display_cell(typed_val.clone())))), true) } else if let Some(typed_val) = any_val.downcast_ref::<example_com_fmt_stringer_imported_types_model::r#mod::Label>() { (Rc::new(RefCell::new(Some(fmt_Stringer::__go_from_display(typed_val.clone())))), true) } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<example_com_fmt_stringer_imported_types_model::r#mod::Label>>>>() { (Rc::new(RefCell::new(Some(fmt_Stringer::__go_from_display_cell(typed_val.clone())))), true) } else { (Rc::new(RefCell::new(None::<fmt_Stringer>)), false) }
        } else {
            (Rc::new(RefCell::new(None::<fmt_Stringer>)), false)
        }
    });
    let mut w: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(example_com_fmt_stringer_imported_types_model::r#mod::Label { text: Rc::new(RefCell::new(Some("y".to_string()))), ..Default::default() }) as Box<dyn Any>)));
    let (mut st, mut ok2) = ({
        let val = w.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<fmt_Stringer>() { (Rc::new(RefCell::new(Some(typed_val.clone()))), true) } else if let Some(typed_val) = any_val.downcast_ref::<Celsius>() { (Rc::new(RefCell::new(Some(fmt_Stringer::__go_from_display(typed_val.clone())))), true) } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Celsius>>>>() { (Rc::new(RefCell::new(Some(fmt_Stringer::__go_from_display_cell(typed_val.clone())))), true) } else if let Some(typed_val) = any_val.downcast_ref::<example_com_fmt_stringer_imported_types_model::r#mod::Label>() { (Rc::new(RefCell::new(Some(fmt_Stringer::__go_from_display(typed_val.clone())))), true) } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<example_com_fmt_stringer_imported_types_model::r#mod::Label>>>>() { (Rc::new(RefCell::new(Some(fmt_Stringer::__go_from_display_cell(typed_val.clone())))), true) } else { (Rc::new(RefCell::new(None::<fmt_Stringer>)), false) }
        } else {
            (Rc::new(RefCell::new(None::<fmt_Stringer>)), false)
        }
    });
    println!("{} {} {}", format!("{}", ok), format!("{}", ok2), format!("{}", match st.borrow().as_ref() { Some(__iface) => format!("{}", __iface), None => "<nil>".to_string() }));
}

impl GoValueClone for Celsius {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
