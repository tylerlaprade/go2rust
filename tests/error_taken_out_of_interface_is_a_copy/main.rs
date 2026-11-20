use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};


#[derive(Clone, Copy)]
struct GoAnyTypeMetadata {
    pub kind: &'static str,
    pub comparable: bool,
    pub elem_kind: Option<&'static str>,
    pub elem_comparable: bool,
}

fn go_any_type_metadata_registry() -> &'static std::sync::Mutex<std::collections::HashMap<std::any::TypeId, GoAnyTypeMetadata>> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<std::any::TypeId, GoAnyTypeMetadata>>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn go_any_value_metadata_registry() -> &'static std::sync::Mutex<std::collections::HashMap<usize, GoAnyTypeMetadata>> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<std::collections::HashMap<usize, GoAnyTypeMetadata>>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()))
}

fn go_any_value_metadata_key(value: &dyn Any) -> usize {
    value as *const dyn Any as *const () as usize
}

fn go_register_any_type<T: Any + 'static>(kind: &'static str, comparable: bool) {
    go_any_type_metadata_registry().lock().unwrap().insert(std::any::TypeId::of::<T>(), GoAnyTypeMetadata { kind, comparable, elem_kind: None, elem_comparable: false });
}

fn go_register_any_type_with_elem<T: Any + 'static>(kind: &'static str, comparable: bool, elem_kind: &'static str, elem_comparable: bool) {
    go_any_type_metadata_registry().lock().unwrap().insert(std::any::TypeId::of::<T>(), GoAnyTypeMetadata { kind, comparable, elem_kind: Some(elem_kind), elem_comparable });
}

// Boxes value as itself and records what kind of Go type it is. Every reader
// of an interface value looks for the value's own type in the box, so the
// box must hold nothing else: a wrapper around the value would be a type no
// reader knows.
fn go_box_any_with_metadata<T: Any + 'static>(value: T, kind: &'static str, comparable: bool) -> Box<dyn Any> {
    go_register_any_type::<T>(kind, comparable);
    Box::new(value) as Box<dyn Any>
}

fn go_register_any_value_metadata(value: &dyn Any, kind: &'static str, comparable: bool) {
    go_any_value_metadata_registry().lock().unwrap().insert(go_any_value_metadata_key(value), GoAnyTypeMetadata { kind, comparable, elem_kind: None, elem_comparable: false });
}

fn go_any_type_metadata(value: &dyn Any) -> Option<GoAnyTypeMetadata> {
    go_any_value_metadata_registry().lock().unwrap().get(&go_any_value_metadata_key(value)).copied()
        .or_else(|| go_any_type_metadata_registry().lock().unwrap().get(&value.type_id()).copied())
}

fn __go_type_name(val: &dyn Any) -> &'static str {
    if val.is::<i64>() { return "int" }
    if val.is::<i32>() { return "int32" }
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
    std::any::type_name_of_val(val)
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct errors_errorString;

impl std::fmt::Display for errors_errorString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<errors_errorString>")
    }
}

impl std::error::Error for errors_errorString {}


impl errors_errorString {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        panic!("errors_errorString.error bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct errors_joinError;

impl std::fmt::Display for errors_joinError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<errors_joinError>")
    }
}

impl std::error::Error for errors_joinError {}


impl errors_joinError {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        panic!("errors_joinError.error bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


fn main() {
    let mut boxed: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some({ let __err_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("x".to_string()))))).clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } })));

    let (mut got, mut ok) = ({
        let val = boxed.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<std::string::String>() {
                (Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(typed_val.clone()))))), true)
            } else {
                (Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(std::string::String::new()))))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(std::string::String::new()))))), false)
        }
    });
    println!("{} {}", format!("{}", ok), format!("{}", { let __text = match got.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    *got.borrow_mut() = None;
    let (mut again, mut ok) = ({
        let val = boxed.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<std::string::String>() {
                (Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(typed_val.clone()))))), true)
            } else {
                (Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(std::string::String::new()))))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(std::string::String::new()))))), false)
        }
    });
    println!("{} {} {}", format!("{}", ok), format!("{}", { let __text = match again.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __text = match got.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

    let mut single = Rc::new(RefCell::new(Some(({
        let val = boxed.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            { if let Some(__err_handle) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() { (*__err_handle.borrow()).clone().expect("type assertion failed") } else { let typed_val = any_val.downcast_ref::<std::string::String>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "error")); Rc::<dyn StdError>::from(Box::<dyn StdError>::from(typed_val.clone())) } }
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "error")
        }
    }))));
    println!("{}", format!("{}", { let __text = match single.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    { let __rhs_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("replaced".to_string()))))).clone(); let new_val = (*__rhs_holder.borrow()).clone(); *single.borrow_mut() = new_val; };
    let (mut third, mut ok) = ({
        let val = boxed.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<std::string::String>() {
                (Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(typed_val.clone()))))), true)
            } else {
                (Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(std::string::String::new()))))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(std::string::String::new()))))), false)
        }
    });
    println!("{} {} {}", format!("{}", ok), format!("{}", { let __text = match third.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __text = match single.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

    {
    let _ts_subject = boxed.clone();
    let _ts_guard = _ts_subject.borrow();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| {
        let mut __any = __v.as_ref() as &dyn Any;
        while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any>>() {
            __any = __boxed.as_ref() as &dyn Any;
        }
        __any
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>()).is_some() {
        let mut v = { let __err_source = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>()).unwrap(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) };
        drop(_ts_guard);
        println!("{} {}", format!("{}", "case error:".to_string()), format!("{}", { let __text = match v.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));;
        *v.borrow_mut() = None;;
        println!("{}", format!("{}", { let __nil_result = (*v.borrow()).is_none(); __nil_result }));;
    }
    }
    let (mut fourth, mut ok) = ({
        let val = boxed.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<std::string::String>() {
                (Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(typed_val.clone()))))), true)
            } else {
                (Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(std::string::String::new()))))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(std::string::String::new()))))), false)
        }
    });
    println!("{} {}", format!("{}", ok), format!("{}", { let __text = match fourth.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

        // The copy is still the same error.
    let mut sentinel = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("sentinel".to_string())))));
    let mut held: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some({ let __err_holder = sentinel.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } })));
    let mut back = Rc::new(RefCell::new(Some(({
        let val = held.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            { if let Some(__err_handle) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() { (*__err_handle.borrow()).clone().expect("type assertion failed") } else { let typed_val = any_val.downcast_ref::<std::string::String>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "error")); Rc::<dyn StdError>::from(Box::<dyn StdError>::from(typed_val.clone())) } }
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "error")
        }
    }))));
    println!("{} {}", format!("{}", { let __left = back.clone(); let __right = sentinel.clone(); let __same_handle = Rc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.borrow(); let __right_guard = __right.borrow(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }), format!("{}", { let __text = match back.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

        // A formatted error takes the same paths.
    let mut wrapped: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some({ let __err_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(format!("wrapped: {}", 7)))))).clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } })));
    let mut w = Rc::new(RefCell::new(Some(({
        let val = wrapped.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            { if let Some(__err_handle) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() { (*__err_handle.borrow()).clone().expect("type assertion failed") } else { let typed_val = any_val.downcast_ref::<std::string::String>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "error")); Rc::<dyn StdError>::from(Box::<dyn StdError>::from(typed_val.clone())) } }
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "error")
        }
    }))));
    let (mut w2, mut ok) = ({
        let val = wrapped.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<std::string::String>() {
                (Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(typed_val.clone()))))), true)
            } else {
                (Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(std::string::String::new()))))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(std::string::String::new()))))), false)
        }
    });
    println!("{} {} {}", format!("{}", { let __text = match w.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __text = match w2.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", ok));
    *w.borrow_mut() = None;
    let mut w3 = Rc::new(RefCell::new(Some(({
        let val = wrapped.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            { if let Some(__err_handle) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() { (*__err_handle.borrow()).clone().expect("type assertion failed") } else { let typed_val = any_val.downcast_ref::<std::string::String>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "error")); Rc::<dyn StdError>::from(Box::<dyn StdError>::from(typed_val.clone())) } }
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "error")
        }
    }))));
    println!("{} {}", format!("{}", { let __nil_result = (*w.borrow()).is_none(); __nil_result }), format!("{}", { let __text = match w3.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
}