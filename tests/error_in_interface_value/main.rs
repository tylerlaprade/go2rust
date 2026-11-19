use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
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
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
        match (*v.borrow()).as_ref() { Some(__err) => format!("{}", __err), None => "<nil>".to_string() }
    } else {
        "<unknown>".to_string()
    }
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


pub type failure = Rc<RefCell<Option<Rc<dyn StdError>>>>;


/// show and count take their arguments as interface values, the way a logging
/// helper does. An error made by errors.New or fmt.Errorf has to survive that.
pub fn show(args: Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) {
    { let __range_holder = args.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.iter().map(|__e| go_any_clone(__e.as_ref())).collect::<Vec<_>>()).unwrap_or_default(); drop(__range_guard); for arg in __range_values.iter() {
        println!("{}", format!("{}", format_any(arg.as_ref())));
    } }
}

pub fn count(args: Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> i32 {
    (*args.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32
}

/// kind tells values apart with a type switch, the way a handler does.
pub fn kind(v: Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<String>>> {
    {
    let _ts_subject = v.clone();
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
        let x = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<i32>()).unwrap().clone())));
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("int {}", { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<String>()).is_some() {
        let x = Rc::new(RefCell::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<String>()).unwrap().clone())));
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("{}{}", "string ".to_string(), (*x.borrow().as_ref().unwrap())))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>()).unwrap().clone();
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("{}{}", "error ".to_string(), (*Rc::new(RefCell::new(Some(format!("{}", x.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())))));;
    } else {
        let x = _ts_subject.clone();
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some("other".to_string())));;
    }
    }
    unreachable!()
}

pub fn is_error(v: Rc<RefCell<Option<Box<dyn Any>>>>) -> bool {
    {
    let _ts_subject = v.clone();
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
        drop(_ts_guard);
        return true;;
    }
    }
    false
}

fn main() {
    let mut plain = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("bad".to_string())))));
    let mut wrapped = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(format!("wrapped: {}", { let __text = match plain.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }))))));
    show(Rc::new(RefCell::new(Some(vec![Box::new("first".to_string()) as Box<dyn Any>, { let __err_holder = plain.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } }, { let __err_holder = wrapped.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } }, Box::new(3) as Box<dyn Any>]))));
    println!("{}", format!("{}", count(Rc::new(RefCell::new(Some(vec![{ let __err_holder = plain.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } }, { let __err_holder = wrapped.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } }]))))));

        // Putting an error in an interface value does not turn it into a string,
        // and taking it back out gives the same error.
    let mut boxed: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some({ let __err_holder = plain.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } })));
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
    println!("{} {}", format!("{}", "is a string:".to_string()), format!("{}", isString));
    {
        let (mut err, mut ok) = ({
        let val = boxed.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
                (typed_val.clone(), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<std::string::String>() {
                (Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(typed_val.clone()))))), true)
            } else {
                (Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(std::string::String::new()))))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(std::string::String::new()))))), false)
        }
    });;
        if ok {
            println!("{} {} {}", format!("{}", "asserted:".to_string()), format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __left = err.clone(); let __right = plain.clone(); let __same_handle = Rc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.borrow(); let __right_guard = __right.borrow(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }));;
        }
    }
    println!("{}", format!("{}", format_any(boxed.borrow().as_ref().unwrap().as_ref())));
    print!("{}|{}\n", format_any(boxed.borrow().as_ref().unwrap().as_ref()), format_any(boxed.borrow().as_ref().unwrap().as_ref()));

    let mut aliased: failure = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("aliased".to_string())))));
    let mut boxedAlias: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some({ let __err_holder = aliased.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } })));
    let (_, mut aliasIsError) = ({
        let val = boxedAlias.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
                (typed_val.clone(), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<std::string::String>() {
                (Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(typed_val.clone()))))), true)
            } else {
                (Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(std::string::String::new()))))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(std::string::String::new()))))), false)
        }
    });
    println!("{} {}", format!("{}", format_any(boxedAlias.borrow().as_ref().unwrap().as_ref())), format!("{}", aliasIsError));

    let mut items = Rc::new(RefCell::new(Some(vec![{ let __err_holder = plain.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } }, Box::new("text".to_string()) as Box<dyn Any>, { let __err_holder = wrapped.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } }])));
    { let __range_holder = items.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.iter().map(|__e| go_any_clone(__e.as_ref())).collect::<Vec<_>>()).unwrap_or_default(); drop(__range_guard); for item in __range_values.iter() {
        println!("{}", format!("{}", format_any(item.as_ref())));
    } }

    let mut number: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(5) as Box<dyn Any>)));
    let (_, mut numberIsError) = ({
        let val = number.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
                (typed_val.clone(), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<std::string::String>() {
                (Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(typed_val.clone()))))), true)
            } else {
                (Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(std::string::String::new()))))), false)
            }
        } else {
            (Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(std::string::String::new()))))), false)
        }
    });
    println!("{}", format!("{}", numberIsError));

        // A type switch finds the error as well, and its case variable is the
        // error.
    println!("{} {} {} {} {} {} {} {} {}", format!("{}", (*kind(Rc::new(RefCell::new(Some({ let __err_holder = plain.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } })))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*kind(Rc::new(RefCell::new(Some({ let __err_holder = wrapped.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } })))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*kind(Rc::new(RefCell::new(Some(Box::new(3) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*kind(Rc::new(RefCell::new(Some(Box::new("s".to_string()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*kind(Rc::new(RefCell::new(Some(Box::new(2.5) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    println!("{} {} {}", format!("{}", is_error(Rc::new(RefCell::new(Some({ let __err_holder = plain.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } }))))), format!("{}", is_error(Rc::new(RefCell::new(Some(Box::new("bad".to_string()) as Box<dyn Any>))))), format!("{}", is_error(Rc::new(RefCell::new(Some(Box::new(4) as Box<dyn Any>))))));
}