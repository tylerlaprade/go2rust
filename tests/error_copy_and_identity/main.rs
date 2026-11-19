use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
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
    } else if let Some(v) = value.downcast_ref::<Missing>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
        match (*v.borrow()).as_ref() { Some(__err) => format!("{}", __err), None => "<nil>".to_string() }
    } else {
        "<unknown>".to_string()
    }
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


#[derive(Clone)]
pub struct Reader {
    pub err: Rc<RefCell<Option<Rc<dyn StdError>>>>,
    pub read: Rc<RefCell<Option<i32>>>,
}

impl Reader {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.err.clone();
        let __go_clone_1_0 = { let __guard = self.read.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            err: __go_clone_0_0,
            read: __go_clone_1_0,
        }
    }
}


impl Default for Reader {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(None));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            err: __go_default_0_0,
            read: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Reader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", match self.err.borrow().as_ref() { Some(__v) => format!("{}", __v), None => "<nil>".to_string() });
        let __go_fmt_1 = format!("{}", (*self.read.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub(crate) struct GoGlobal<T> {
    value: std::cell::UnsafeCell<Option<T>>,
}
unsafe impl<T> Sync for GoGlobal<T> {}
impl<T> GoGlobal<T> {
    pub(crate) const fn new() -> Self {
        Self { value: std::cell::UnsafeCell::new(None) }
    }
    pub(crate) fn borrow(&'static self) -> &'static Option<T> {
        unsafe { &*self.value.get() }
    }
    pub(crate) fn borrow_mut(&'static self) -> &'static mut Option<T> {
        unsafe { &mut *self.value.get() }
    }
    pub(crate) fn clone(&'static self) -> std::rc::Rc<std::cell::RefCell<Option<T>>> where T: Clone {
        std::rc::Rc::new(std::cell::RefCell::new(self.borrow().clone()))
    }
}

pub static ErrNotFound: GoGlobal<Rc<dyn StdError>> = GoGlobal::new();

pub static ErrClosed: GoGlobal<Rc<dyn StdError>> = GoGlobal::new();


fn __go_init_globals() {
    *ErrNotFound.borrow_mut() = None;
    *ErrClosed.borrow_mut() = None;
    { let __rhs_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("not found".to_string()))))).clone(); let new_val = (*__rhs_holder.borrow()).clone(); *ErrNotFound.borrow_mut() = new_val; }
    { let __rhs_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("closed".to_string()))))).clone(); let new_val = (*__rhs_holder.borrow()).clone(); *ErrClosed.borrow_mut() = new_val; }
}


impl Missing {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("{}{}", "missing ".to_string(), (*self.name.clone().borrow().as_ref().unwrap())))))
    }
}

impl StdError for Missing {}


impl Reader {
    pub fn err(&self) -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
        { let __err_source = { let __field = self.err.clone(); __field }; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }
    }

    pub fn fail(&mut self, err: Rc<RefCell<Option<Rc<dyn StdError>>>>) {
        { let __rhs_holder = err.clone(); let new_val = (*__rhs_holder.borrow()).clone(); *self.err.borrow_mut() = new_val; };
    }
}

pub fn run(fail: Rc<RefCell<Option<bool>>>) -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
    if (*fail.borrow().as_ref().unwrap()) {
        return Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("boom".to_string())))));
    }
    Rc::new(RefCell::new(None))
}

pub fn find(key: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
    if (*key.borrow().as_ref().unwrap()).clone() == "".to_string() {
        return { let __err_source = ErrNotFound.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) };
    }
    if (*key.borrow().as_ref().unwrap()).clone() == "closed".to_string() {
        return { let __err_source = ErrClosed.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) };
    }
    Rc::new(RefCell::new(None))
}

pub fn lookup(key: Rc<RefCell<Option<String>>>) -> (i32, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
    {
        let mut err = find(Rc::new(RefCell::new(Some({ let __arg_holder = key.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
            return (0, { let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) });;
        }
    }
    ((*key.borrow().as_ref().unwrap()).len() as i32, Rc::new(RefCell::new(None)))
}

pub fn report(mut err: Rc<RefCell<Option<Rc<dyn StdError>>>>) {
    println!("{} {}", format!("{}", "report:".to_string()), format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    *err.borrow_mut() = None;
}

pub fn classify(err: Rc<RefCell<Option<Rc<dyn StdError>>>>) -> Rc<RefCell<Option<String>>> {
    { let _switch_val = { let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) };
    if (*_switch_val.borrow()).is_none() {
            return Rc::new(RefCell::new(Some("ok".to_string())));
        } else if { let __left_holder = _switch_val.clone(); let __left_guard = __left_holder.borrow(); let __right_holder = { let __err_source = ErrNotFound.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __right_guard = __right_holder.borrow(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__left), Some(__right)) => std::ptr::addr_eq(&**__left, &**__right), (None, None) => true, _ => false } } {
            return Rc::new(RefCell::new(Some("not found".to_string())));
        } else if { let __left_holder = _switch_val.clone(); let __left_guard = __left_holder.borrow(); let __right_holder = { let __err_source = ErrClosed.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }; let __right_guard = __right_holder.borrow(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__left), Some(__right)) => std::ptr::addr_eq(&**__left, &**__right), (None, None) => true, _ => false } } {
            return Rc::new(RefCell::new(Some("closed".to_string())));
        }
    }
    Rc::new(RefCell::new(Some(format!("{}{}", "other: ".to_string(), (*Rc::new(RefCell::new(Some(format!("{}", err.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())))))
}

/// work replaces its own result in a deferred function.
pub fn work(fail: Rc<RefCell<Option<bool>>>) -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut err: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(None));

    let mut err_defer_captured = err.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Rc::new(RefCell::new(Some(Box::new(move || {
        if { let __nil_result = (*err_defer_captured.borrow()).is_some(); __nil_result } {
        { let __rhs_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(format!("wrapped: {}", { let __text = match err_defer_captured.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text })))))).clone(); let new_val = (*__rhs_holder.borrow()).clone(); *err_defer_captured.borrow_mut() = new_val; };
    }
    }) as Box<dyn FnMut() -> ()>))); let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = __f_holder.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    if (*fail.borrow().as_ref().unwrap()) {
        {
        { let __rhs_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("inner".to_string()))))).clone(); let new_val = (*__rhs_holder.borrow()).clone(); *err.borrow_mut() = new_val; };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return err.clone();
    }
    }
    {
        *err.borrow_mut() = None;;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return err.clone();
    }
}

fn main() {
    __go_init_all();
        // Assigning an error leaves the one it was assigned from.
    let mut last: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(None));
    let mut err = run(Rc::new(RefCell::new(Some(true))));
    if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
        { let __rhs_holder = err.clone(); let new_val = (*__rhs_holder.borrow()).clone(); *last.borrow_mut() = new_val; };
    }
    println!("{} {}", format!("{}", { let __nil_result = (*err.borrow()).is_some(); __nil_result }), format!("{}", { let __nil_result = (*last.borrow()).is_some(); __nil_result }));
    println!("{} {}", format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __text = match last.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

        // A copy is a variable of its own.
    let mut e1 = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("first".to_string())))));
    let mut e2 = { let __err_source = e1.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) };
    { let __rhs_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("second".to_string()))))).clone(); let new_val = (*__rhs_holder.borrow()).clone(); *e1.borrow_mut() = new_val; };
    println!("{} {}", format!("{}", { let __text = match e1.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __text = match e2.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

        // A package-level error can be returned, compared and returned again.
    println!("{} {} {}", format!("{}", { let __text = match find(Rc::new(RefCell::new(Some("".to_string())))).borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __text = match find(Rc::new(RefCell::new(Some("closed".to_string())))).borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __text = match find(Rc::new(RefCell::new(Some("x".to_string())))).borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    println!("{} {} {}", format!("{}", { let __left = find(Rc::new(RefCell::new(Some("".to_string())))).clone(); let __right = ErrNotFound.clone(); let __same_handle = Rc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.borrow(); let __right_guard = __right.borrow(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }), format!("{}", { let __left = find(Rc::new(RefCell::new(Some("".to_string())))).clone(); let __right = ErrClosed.clone(); let __same_handle = Rc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.borrow(); let __right_guard = __right.borrow(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }), format!("{}", { let __nil_result = (*find(Rc::new(RefCell::new(Some("x".to_string())))).borrow()).is_none(); __nil_result }));
    println!("{} {} {} {}", format!("{}", (*classify(find(Rc::new(RefCell::new(Some("".to_string()))))).borrow().as_ref().unwrap())), format!("{}", (*classify(find(Rc::new(RefCell::new(Some("closed".to_string()))))).borrow().as_ref().unwrap())), format!("{}", (*classify(find(Rc::new(RefCell::new(Some("x".to_string()))))).borrow().as_ref().unwrap())), format!("{}", (*classify({ let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }).borrow().as_ref().unwrap())));
    {
        let (mut n, mut err) = lookup(Rc::new(RefCell::new(Some("".to_string()))));;
        if { let __left = err.clone(); let __right = ErrNotFound.clone(); let __same_handle = Rc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.borrow(); let __right_guard = __right.borrow(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq } {
            println!("{} {} {}", format!("{}", "lookup:".to_string()), format!("{}", n), format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));;
        }
    }
    let mut got = find(Rc::new(RefCell::new(Some("".to_string()))));
    *got.borrow_mut() = None;
    println!("{} {} {}", format!("{}", { let __nil_result = (*got.borrow()).is_none(); __nil_result }), format!("{}", { let __text = match ErrNotFound.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __left = find(Rc::new(RefCell::new(Some("".to_string())))).clone(); let __right = ErrNotFound.clone(); let __same_handle = Rc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.borrow(); let __right_guard = __right.borrow(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }));

        // Two errors with the same text are two errors. A copy is the same error.
    let mut a = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("same".to_string())))));
    let mut b = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("same".to_string())))));
    let mut c = { let __err_source = a.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) };
    println!("{} {} {} {}", format!("{}", { let __left = a.clone(); let __right = b.clone(); let __same_handle = Rc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.borrow(); let __right_guard = __right.borrow(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }), format!("{}", { let __left = a.clone(); let __right = c.clone(); let __same_handle = Rc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.borrow(); let __right_guard = __right.borrow(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }), format!("{}", { let __left = a.clone(); let __right = b.clone(); let __same_handle = Rc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.borrow(); let __right_guard = __right.borrow(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; !__eq }), format!("{}", { let __left = c.clone(); let __right = a.clone(); let __same_handle = Rc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.borrow(); let __right_guard = __right.borrow(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }));

        // Passing an error does not give the variable away, and the function
        // cannot change it.
    report({ let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) });
    report({ let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) });
    println!("{}", format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

        // An error in a struct field.
    let mut r = Rc::new(RefCell::new(Some(Reader { err: Default::default(), read: Rc::new(RefCell::new(Some(0))) })));
    (*r.borrow_mut().as_mut().unwrap()).fail({ let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) });
    let mut held = (*r.borrow().as_ref().unwrap()).err();
    *held.borrow_mut() = None;
    println!("{} {} {} {}", format!("{}", { let __nil_result = (*held.borrow()).is_none(); __nil_result }), format!("{}", { let __text = match (*r.borrow().as_ref().unwrap()).err().borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __left = (*r.borrow().as_ref().unwrap()).err.clone(); let __right = err.clone(); let __same_handle = Rc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.borrow(); let __right_guard = __right.borrow(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }), format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    { let __rhs_holder = ErrClosed.clone(); let new_val = (*__rhs_holder.borrow()).clone(); *(*r.borrow().as_ref().unwrap()).err.borrow_mut() = new_val; };
    println!("{} {}", format!("{}", { let __left = (*r.borrow().as_ref().unwrap()).err().clone(); let __right = ErrClosed.clone(); let __same_handle = Rc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.borrow(); let __right_guard = __right.borrow(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }), format!("{}", (*classify((*r.borrow().as_ref().unwrap()).err()).borrow().as_ref().unwrap())));

        // Errors in a slice and in a map.
    let mut list = Rc::new(RefCell::new(Some(vec![{ let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, { let __err_source = last.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, { let __err_source = ErrNotFound.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))])));
    { let __range_holder = list.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, item) in __range_values.iter().cloned().enumerate() {
        println!("{} {} {}", format!("{}", i), format!("{}", { let __text = match item.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __left = item.clone(); let __right = err.clone(); let __same_handle = Rc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.borrow(); let __right_guard = __right.borrow(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }));
    } }
    { let new_val = { let __append_target = list.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push({ let __err_source = e2.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }); __append_target.clone() }; list = new_val; };
    println!("{} {} {}", format!("{}", (*list.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", { let __text = match (*list.borrow().as_ref().unwrap())[(4) as usize].clone().borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    let mut byName = Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<Rc<dyn StdError>>>>>::from([("a".to_string(), { let __err_source = err.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }), ("b".to_string(), { let __err_source = ErrClosed.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) })]))));
    println!("{} {} {}", format!("{}", { let __text = match { let __map_holder = byName.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __left = { let __map_holder = byName.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"b".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __right = ErrClosed.clone(); let __same_handle = Rc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.borrow(); let __right_guard = __right.borrow(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }), format!("{}", { let __nil_result = (*{ let __map_holder = byName.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().and_then(|__map| __map.get(&"c".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.borrow()).is_none(); __nil_result }));

        // A custom error type.
    let mut m: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::new(Missing { name: Rc::new(RefCell::new(Some("key".to_string()))), ..Default::default() }) as Rc<dyn StdError>)));
    let mut n = { let __err_source = m.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) };
    *m.borrow_mut() = None;
    println!("{} {}", format!("{}", { let __nil_result = (*m.borrow()).is_none(); __nil_result }), format!("{}", { let __text = match n.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

    *last.borrow_mut() = None;
    println!("{} {}", format!("{}", { let __nil_result = (*last.borrow()).is_none(); __nil_result }), format!("{}", { let __nil_result = (*err.borrow()).is_some(); __nil_result }));

        // An interface value holds a copy as well: it does not follow the
        // variable it was made from.
    let mut boxed: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some({ let __err_holder = err.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<Missing>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else { Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } } })));
    { let __rhs_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("changed".to_string()))))).clone(); let new_val = (*__rhs_holder.borrow()).clone(); *err.borrow_mut() = new_val; };
    println!("{} {}", format!("{}", format_any(boxed.borrow().as_ref().unwrap().as_ref())), format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));

        // So does what a closure returns.
    let err_closure_clone = err.clone(); let mut check = Rc::new(RefCell::new(Some(Box::new(move || -> Rc<RefCell<Option<Rc<dyn StdError>>>> {
        return { let __err_source = err_closure_clone.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) };
    }) as Box<dyn FnMut() -> Rc<RefCell<Option<Rc<dyn StdError>>>>>)));
    let mut fromClosure = { let __f_ptr: *mut Box<dyn FnMut() -> Rc<RefCell<Option<Rc<dyn StdError>>>>> = { let mut __f_guard = check.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Rc<RefCell<Option<Rc<dyn StdError>>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    *fromClosure.borrow_mut() = None;
    println!("{} {} {}", format!("{}", { let __nil_result = (*fromClosure.borrow()).is_none(); __nil_result }), format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __left = { let __f_ptr: *mut Box<dyn FnMut() -> Rc<RefCell<Option<Rc<dyn StdError>>>>> = { let mut __f_guard = check.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Rc<RefCell<Option<Rc<dyn StdError>>>>> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() }.clone(); let __right = err.clone(); let __same_handle = Rc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.borrow(); let __right_guard = __right.borrow(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false } }; __eq }));

        // A deferred function can still replace a named result.
    println!("{} {}", format!("{}", { let __text = match work(Rc::new(RefCell::new(Some(true)))).borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __text = match work(Rc::new(RefCell::new(Some(false)))).borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
