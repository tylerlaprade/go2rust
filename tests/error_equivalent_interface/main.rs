use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
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
    if val.is::<customError>() { return "main.customError" }
    std::any::type_name_of_val(val)
}

// An error that wraps others: fmt.Errorf with %w, or errors.Join. many is
// true when it wraps more than one, and then errors.Unwrap does not see it.
#[derive(Debug)]
struct GoWrapError {
    msg: String,
    wrapped: Vec<Rc<dyn StdError>>,
    many: bool,
}

impl std::fmt::Display for GoWrapError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl StdError for GoWrapError {}

fn go_error_wrap(msg: String, wrapped: Vec<Option<Rc<dyn StdError>>>) -> Rc<dyn StdError> {
    let many = wrapped.len() > 1;
    Rc::new(GoWrapError { msg, wrapped: wrapped.into_iter().flatten().collect(), many }) as Rc<dyn StdError>
}

fn go_errors_join(errs: Vec<Option<Rc<dyn StdError>>>) -> Option<Rc<dyn StdError>> {
    let errs: Vec<Rc<dyn StdError>> = errs.into_iter().flatten().collect();
    if errs.is_empty() {
        return None;
    }
    let msg = errs.iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n");
    Some(Rc::new(GoWrapError { msg, wrapped: errs, many: true }) as Rc<dyn StdError>)
}

// err and every error it wraps, depth first, in the order errors.Is and
// errors.As visit them.
fn go_error_chain(err: Option<Rc<dyn StdError>>) -> Vec<Rc<dyn StdError>> {
    let mut out = Vec::new();
    let mut stack: Vec<Rc<dyn StdError>> = err.into_iter().collect();
    while let Some(e) = stack.pop() {
        if let Some(w) = e.downcast_ref::<GoWrapError>() {
            stack.extend(w.wrapped.iter().rev().cloned());
        }
        out.push(e);
    }
    out
}

fn go_errors_unwrap(err: Option<Rc<dyn StdError>>) -> Option<Rc<dyn StdError>> {
    let err = err?;
    if let Some(w) = err.downcast_ref::<GoWrapError>() {
        return if w.many { None } else { w.wrapped.first().cloned() };
    }
    let e = err;
    let mut stack: Vec<Rc<dyn StdError>> = Vec::new();
    stack.pop()
}

// Two errors are the same error when they point to the same value, as ==
// on two errors is written.
fn go_errors_is(err: Option<Rc<dyn StdError>>, target: Option<Rc<dyn StdError>>) -> bool {
    match (err, target) {
        (None, None) => true,
        (Some(err), Some(target)) => go_error_chain(Some(err)).iter().any(|e| std::ptr::addr_eq(&**e, &*target)),
        _ => false,
    }
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


pub trait failer: std::fmt::Display + Any {
    fn __go_clone_box_failer(&self) -> Box<dyn failer>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_failer(&self, other: &dyn failer) -> bool;
    fn error(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn failer> {
    fn clone(&self) -> Self {
        failer::__go_clone_box_failer(self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct customError {
    pub text: Rc<RefCell<Option<String>>>,
}

impl customError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.text.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            text: __go_clone_0_0,
        }
    }
}


impl Default for customError {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            text: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for customError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


impl customError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some({ let __selector_holder = self.text.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }
}

impl StdError for customError {}


impl failer for customError {
    fn error(&self) -> Rc<RefCell<Option<String>>> {
        customError::error(self)
    }
    fn __go_clone_box_failer(&self) -> Box<dyn failer> {
        Box::new(self.clone()) as Box<dyn failer>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_failer(&self, other: &dyn failer) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<customError>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct customErrorPtr(pub Rc<RefCell<Option<customError>>>);

impl std::fmt::Display for customErrorPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl failer for customErrorPtr {
    fn error(&self) -> Rc<RefCell<Option<String>>> {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        customError::error(__recv)
    }
    fn __go_clone_box_failer(&self) -> Box<dyn failer> {
        Box::new(self.clone()) as Box<dyn failer>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_failer(&self, other: &dyn failer) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<customErrorPtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn describe(f: Rc<RefCell<Option<Rc<dyn StdError>>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some(format!("{}", f.borrow().as_ref().unwrap()))))
}

pub fn switch_named(v: Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<String>>> {
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
        let x = { let __err_source = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>()).unwrap(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) };
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("{}", x.borrow().as_ref().unwrap()))));;
    } else {
        let x = _ts_subject.clone();
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some("default".to_string())));;
    }
    }
    unreachable!()
}

pub fn switch_anonymous(v: Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<String>>> {
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
        let x = { let __err_source = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>()).unwrap(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) };
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some(format!("{}", x.borrow().as_ref().unwrap()))));;
    } else {
        let x = _ts_subject.clone();
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some("default".to_string())));;
    }
    }
    unreachable!()
}

fn main() {
    let mut direct: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("x".to_string())))));
    println!("{} {}", format!("{}", "direct".to_string()), format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", direct.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "call".to_string()), format!("{}", (*describe(Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("z".to_string())))))).borrow().as_ref().unwrap())));
    let mut custom: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::new(customError { text: Rc::new(RefCell::new(Some("custom".to_string()))), ..Default::default() }) as Rc<dyn StdError>)));
    println!("{} {}", format!("{}", "custom".to_string()), format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", custom.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));

    let mut anonymous: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("y".to_string())))));
    println!("{} {}", format!("{}", "anonymous".to_string()), format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", anonymous.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));
    let mut embedded: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("embedded".to_string())))));
    println!("{} {}", format!("{}", "embedded".to_string()), format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", embedded.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));

    let mut value: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some({ let __err_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("bad".to_string()))))).clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<customError>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else { Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } } })));
    let (mut asserted, mut ok) = ({
        let val = value.clone();
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
    println!("{} {} {}", format!("{}", "assert".to_string()), format!("{}", ok), format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", asserted.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));
    let mut single = Rc::new(RefCell::new(Some(({
        let val = value.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            { if let Some(__err_handle) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() { (*__err_handle.borrow()).clone().expect("type assertion failed") } else { let typed_val = any_val.downcast_ref::<std::string::String>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "main.failer")); Rc::<dyn StdError>::from(Box::<dyn StdError>::from(typed_val.clone())) } }
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "main.failer")
        }
    }))));
    println!("{} {}", format!("{}", "single".to_string()), format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", single.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "switch named".to_string()), format!("{}", (*switch_named(value.clone()).borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "switch anonymous".to_string()), format!("{}", (*switch_anonymous(value.clone()).borrow().as_ref().unwrap())));

    let mut wrapped = Rc::new(RefCell::new(Some(go_error_wrap(format!("wrap: {}", match { let __err_handle = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("inner".to_string()))))); let __err_value = (*__err_handle.borrow()).clone(); __err_value } { Some(__wrapped) => format!("{}", __wrapped), None => "%!w(<nil>)".to_string() }), vec![{ let __err_handle = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("inner".to_string()))))); let __err_value = (*__err_handle.borrow()).clone(); __err_value }]))));
    println!("{} {}", format!("{}", "wrapped".to_string()), format!("{}", (*describe({ let __err_source = wrapped.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }).borrow().as_ref().unwrap())));
}