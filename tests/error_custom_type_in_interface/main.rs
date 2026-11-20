use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
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
    } else if let Some(v) = value.downcast_ref::<Missing>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<NotFound>>>>() {
        match (*v.borrow()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() }
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
        match (*v.borrow()).as_ref() { Some(__err) => format!("{}", __err), None => "<nil>".to_string() }
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

fn format_any_slice_values(slice: &Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> String {
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| format_any(v.as_ref())).collect();
        formatted.join(" ")
    } else {
        String::new()
    }
}

fn format_any_slice(slice: &Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> String {
    format!("[{}]", format_any_slice_values(slice))
}

fn format_any_variadic(slice: &Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> String {
    format_any_slice_values(slice)
}

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
    if let Some(v) = value.downcast_ref::<Missing>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<NotFound>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() { return Box::new(v.clone()) as Box<dyn Any>; }

    panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>")
}

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

#[derive(Debug, Clone)]
pub struct Missing {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Missing {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for Missing {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Missing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct NotFound {
    pub code: Rc<RefCell<Option<i32>>>,
}

impl NotFound {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.code.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            code: __go_clone_0_0,
        }
    }
}


impl Default for NotFound {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            code: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for NotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


impl Missing {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("{}{}", "missing ".to_string(), (*self.name.clone().borrow().as_ref().unwrap())))))
    }
}

impl StdError for Missing {}


impl NotFound {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("not found {}", (*self.code.borrow().as_ref().unwrap())))))
    }
}

impl StdError for NotFound {}


/// log and logAll take their arguments as interface values, the way a logging
/// helper does. An error of a type the program defines has to keep its text
/// there, and it has to survive being copied.
pub fn find() -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
    Rc::new(RefCell::new(Some(Rc::new(Missing { name: Rc::new(RefCell::new(Some("key".to_string()))), ..Default::default() }) as Rc<dyn StdError>)))
}

pub fn lookup() -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
    Rc::new(RefCell::new(Some(Rc::new(NotFound { code: Rc::new(RefCell::new(Some(404))), ..Default::default() }) as Rc<dyn StdError>)))
}

pub fn log(v: Rc<RefCell<Option<Box<dyn Any>>>>) {
    println!("{}", format!("{}", format_any(v.borrow().as_ref().unwrap().as_ref())));
    print!("{}|{}\n", format_any(v.borrow().as_ref().unwrap().as_ref()), format_any(v.borrow().as_ref().unwrap().as_ref()));
}

pub fn log_all(args: Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) {
    { let __range_holder = args.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.iter().map(|__e| go_any_clone(__e.as_ref())).collect::<Vec<_>>()).unwrap_or_default(); drop(__range_guard); for arg in __range_values.iter() {
        println!("{}", format!("{}", format_any(arg.as_ref())));
    } }
    println!("{}", format_any_variadic(&args));
}

pub fn keep(v: Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<Box<dyn Any>>>> {
    let mut w = v.clone();
    return w.clone();
}

fn main() {
    let mut err = find();
    log(Rc::new(RefCell::new(Some({ let __err_holder = err.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<Missing>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<NotFound>() { go_box_any_with_metadata(Rc::new(RefCell::new(Some(typed_val.clone()))), "pointer", true) } else { Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } } }))));
    let mut other = lookup();
    log(Rc::new(RefCell::new(Some({ let __err_holder = other.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<Missing>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<NotFound>() { go_box_any_with_metadata(Rc::new(RefCell::new(Some(typed_val.clone()))), "pointer", true) } else { Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } } }))));
    log_all(Rc::new(RefCell::new(Some(vec![Box::new("failed:".to_string()) as Box<dyn Any>, { let __err_holder = err.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<Missing>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<NotFound>() { go_box_any_with_metadata(Rc::new(RefCell::new(Some(typed_val.clone()))), "pointer", true) } else { Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } } }, { let __err_holder = other.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<Missing>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<NotFound>() { go_box_any_with_metadata(Rc::new(RefCell::new(Some(typed_val.clone()))), "pointer", true) } else { Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } } }]))));
    println!("{} {}", format!("{}", format_any((keep(Rc::new(RefCell::new(Some({ let __err_holder = err.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<Missing>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<NotFound>() { go_box_any_with_metadata(Rc::new(RefCell::new(Some(typed_val.clone()))), "pointer", true) } else { Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } } }))))).borrow().as_ref().unwrap().as_ref())), format!("{}", format_any((keep(Rc::new(RefCell::new(Some({ let __err_holder = other.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<Missing>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<NotFound>() { go_box_any_with_metadata(Rc::new(RefCell::new(Some(typed_val.clone()))), "pointer", true) } else { Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } } }))))).borrow().as_ref().unwrap().as_ref())));
    log(Rc::new(RefCell::new(Some(Box::new(Missing { name: Rc::new(RefCell::new(Some("direct".to_string()))), ..Default::default() }) as Box<dyn Any>))));

        // The interface value holds the error under its own type.
    let mut boxed: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some({ let __err_holder = err.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<Missing>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<NotFound>() { go_box_any_with_metadata(Rc::new(RefCell::new(Some(typed_val.clone()))), "pointer", true) } else { Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } } })));
    {
        let (mut m, mut ok) = ({
        let val = boxed.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Missing>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), true)
            } else {
                (Rc::new(RefCell::new(Some(Default::default()))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(Default::default()))), false)
        }
    });;
        if ok {
            println!("{} {}", format!("{}", "missing:".to_string()), format!("{}", (*(*m.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()));;
        }
    }
    let (_, mut isString) = ({
        let val = boxed.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<std::string::String>() {
                (Rc::new(RefCell::new(Some(typed_val.clone()))), true)
            } else {
                (Rc::new(RefCell::new(Some(std::string::String::new()))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(std::string::String::new()))), false)
        }
    });
    let (_, mut isNotFound) = ({
        let val = boxed.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<NotFound>>>>() {
                (typed_val.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<NotFound>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<NotFound>)), false)
        }
    });
    println!("{} {}", format!("{}", isString), format!("{}", isNotFound));
    let mut boxedPointer: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some({ let __err_holder = other.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<Missing>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<NotFound>() { go_box_any_with_metadata(Rc::new(RefCell::new(Some(typed_val.clone()))), "pointer", true) } else { Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } } })));
    {
        let (mut n, mut ok) = ({
        let val = boxedPointer.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<NotFound>>>>() {
                (typed_val.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<NotFound>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<NotFound>)), false)
        }
    });;
        if ok {
            println!("{} {}", format!("{}", "not found:".to_string()), format!("{}", (*(*n.borrow().as_ref().unwrap()).code.borrow().as_ref().unwrap())));;
        }
    }
}