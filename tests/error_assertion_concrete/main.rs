use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};


struct GoPointerError<T> {
    handle: Rc<RefCell<Option<T>>>,
}

impl<T> GoPointerError<T> {
    fn new(handle: Rc<RefCell<Option<T>>>) -> Self { Self { handle } }
}

impl<T: std::fmt::Display> std::fmt::Display for GoPointerError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let guard = self.handle.borrow();
        match guard.as_ref() {
            Some(value) => write!(f, "{}", value),
            None => Ok(()),
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Debug for GoPointerError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl<T: std::fmt::Display + 'static> std::error::Error for GoPointerError<T> {}

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
    if val.is::<Rc<RefCell<Option<notFound>>>>() { return "*main.notFound" }
    if val.is::<Rc<RefCell<Option<pointerError>>>>() { return "*main.pointerError" }
    if val.is::<notFound>() { return "main.notFound" }
    std::any::type_name_of_val(val)
}

#[derive(Debug, Clone)]
pub struct notFound {
    pub name: Rc<RefCell<Option<String>>>,
}

impl notFound {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for notFound {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for notFound {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct pointerError {
    pub code: Rc<RefCell<Option<i64>>>,
}

impl pointerError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.code.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            code: __go_clone_0_0,
        }
    }
}


impl Default for pointerError {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            code: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for pointerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


impl notFound {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("{}{}", (*self.name.clone().borrow().as_ref().unwrap()), " not found".to_string()))));
    }
}

impl StdError for notFound {}


impl pointerError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some(format!("pointer {}", (*self.code.borrow().as_ref().unwrap())))))
    }
}

impl StdError for pointerError {}


pub fn error_text(v: Rc<RefCell<Option<Box<dyn Any>>>>) -> (bool, Rc<RefCell<Option<String>>>) {
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<notFound>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<pointerError>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<notFound>>>>()).is_some() {
        let err = if let Some(__err_handle) = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>()) { { let __err_source = __err_handle; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) } } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<notFound>()) { Rc::new(RefCell::new(Some(Rc::new(typed_val.clone()) as Rc<dyn StdError>))) } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<pointerError>>>>()) { Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))) } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<notFound>>>>()) { Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))) } else { unreachable!("type switch error case matched no error payload") };
        drop(_ts_guard);
        return (true, Rc::new(RefCell::new(Some(format!("{}", err.borrow().as_ref().unwrap())))));;
    } else {
        let err = _ts_subject.clone();
        drop(_ts_guard);
        return (false, Rc::new(RefCell::new(Some("".to_string()))));;
    }
    }
    unreachable!()
}

pub fn replace_then_error_text(v: Rc<RefCell<Option<Box<dyn Any>>>>, p: Rc<RefCell<Option<pointerError>>>) -> Rc<RefCell<Option<String>>> {
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<notFound>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<pointerError>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<notFound>>>>()).is_some() {
        let err = if let Some(__err_handle) = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>()) { { let __err_source = __err_handle; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) } } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<notFound>()) { Rc::new(RefCell::new(Some(Rc::new(typed_val.clone()) as Rc<dyn StdError>))) } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<pointerError>>>>()) { Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))) } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<notFound>>>>()) { Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))) } else { unreachable!("type switch error case matched no error payload") };
        drop(_ts_guard);
        { let new_val = pointerError { code: Rc::new(RefCell::new(Some(10))), ..Default::default() }; *p.borrow_mut() = Some(new_val); };;
        return Rc::new(RefCell::new(Some(format!("{}", err.borrow().as_ref().unwrap()))));;
    } else {
        let err = _ts_subject.clone();
        drop(_ts_guard);
        return Rc::new(RefCell::new(Some("not error".to_string())));;
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<notFound>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<pointerError>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Rc<RefCell<Option<notFound>>>>()).is_some() {
        drop(_ts_guard);
        return true;;
    } else {
        drop(_ts_guard);
        return false;;
    }
    }
    unreachable!()
}

fn main() {
    let mut value: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(notFound { name: Rc::new(RefCell::new(Some("key".to_string()))), ..Default::default() }) as Box<dyn Any>)));
    let (mut valueErr, mut valueOK) = ({
        let val = value.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<notFound>() {
                (Rc::new(RefCell::new(Some(Rc::new(typed_val.clone()) as Rc<dyn StdError>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<pointerError>>>>() {
                (Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<notFound>>>>() {
                (Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))), true)
            } else {
                (Rc::new(RefCell::new(None::<Rc<dyn StdError>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Rc<dyn StdError>>)), false)
        }
    });
    let (mut valueCase, mut valueText) = error_text(value.clone());
    println!("{} {} {} {}", format!("{}", valueOK), format!("{}", valueCase), format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", valueErr.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", ({
        let val = value.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            { if let Some(__err_handle) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() { (*__err_handle.borrow()).clone().expect("type assertion failed") } else if let Some(typed_val) = any_val.downcast_ref::<notFound>() { Rc::new(typed_val.clone()) as Rc<dyn StdError> } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<pointerError>>>>() { Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError> } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<notFound>>>>() { Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError> } else { panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "error") } }
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "error")
        }
    }))))).borrow().as_ref().unwrap())));

    let mut pointerValue = Rc::new(RefCell::new(Some(pointerError { code: Rc::new(RefCell::new(Some(7))), ..Default::default() })));
    let mut pointer: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(pointerValue.clone()) as Box<dyn Any>)));
    let (mut pointerErr, mut pointerOK) = ({
        let val = pointer.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<notFound>() {
                (Rc::new(RefCell::new(Some(Rc::new(typed_val.clone()) as Rc<dyn StdError>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<pointerError>>>>() {
                (Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<notFound>>>>() {
                (Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))), true)
            } else {
                (Rc::new(RefCell::new(None::<Rc<dyn StdError>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Rc<dyn StdError>>)), false)
        }
    });
    let (mut pointerCase, mut pointerText) = error_text(pointer.clone());
    println!("{} {} {} {}", format!("{}", pointerOK), format!("{}", pointerCase), format!("{}", { let __v = (*pointerText.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", ({
        let val = pointer.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            { if let Some(__err_handle) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() { (*__err_handle.borrow()).clone().expect("type assertion failed") } else if let Some(typed_val) = any_val.downcast_ref::<notFound>() { Rc::new(typed_val.clone()) as Rc<dyn StdError> } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<pointerError>>>>() { Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError> } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<notFound>>>>() { Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError> } else { panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "error") } }
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "error")
        }
    }))))).borrow().as_ref().unwrap())));
    { let new_val = pointerError { code: Rc::new(RefCell::new(Some(8))), ..Default::default() }; *pointerValue.borrow_mut() = Some(new_val); };
    println!("{} {}", format!("{}", "assertion replacement".to_string()), format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", pointerErr.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "switch replacement".to_string()), format!("{}", (*replace_then_error_text(pointer.clone(), pointerValue.clone()).borrow().as_ref().unwrap())));

    let mut valuePointer: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(notFound { name: Rc::new(RefCell::new(Some("pointer key".to_string()))), ..Default::default() }))).clone()) as Box<dyn Any>)));
    let (mut valuePointerErr, mut valuePointerOK) = ({
        let val = valuePointer.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<notFound>() {
                (Rc::new(RefCell::new(Some(Rc::new(typed_val.clone()) as Rc<dyn StdError>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<pointerError>>>>() {
                (Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<notFound>>>>() {
                (Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))), true)
            } else {
                (Rc::new(RefCell::new(None::<Rc<dyn StdError>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Rc<dyn StdError>>)), false)
        }
    });
    let (mut valuePointerCase, _) = error_text(valuePointer.clone());
    println!("{} {} {}", format!("{}", valuePointerOK), format!("{}", valuePointerCase), format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", valuePointerErr.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));

    let mut nilPointer: Rc<RefCell<Option<pointerError>>> = Rc::new(RefCell::new(None));
    let mut typedNil: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(nilPointer.clone()) as Box<dyn Any>)));
    let (mut typedNilErr, mut typedNilOK) = ({
        let val = typedNil.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<notFound>() {
                (Rc::new(RefCell::new(Some(Rc::new(typed_val.clone()) as Rc<dyn StdError>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<pointerError>>>>() {
                (Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<notFound>>>>() {
                (Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))), true)
            } else {
                (Rc::new(RefCell::new(None::<Rc<dyn StdError>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Rc<dyn StdError>>)), false)
        }
    });
    let mut typedNilCase = is_error(typedNil.clone());
    println!("{} {} {}", format!("{}", typedNilOK), format!("{}", typedNilCase), format!("{}", { let __nil_result = (*typedNilErr.borrow()).is_none(); __nil_result }));

    let mut text: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new("plain".to_string()) as Box<dyn Any>)));
    let (_, mut textOK) = ({
        let val = text.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<notFound>() {
                (Rc::new(RefCell::new(Some(Rc::new(typed_val.clone()) as Rc<dyn StdError>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<pointerError>>>>() {
                (Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<notFound>>>>() {
                (Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))), true)
            } else {
                (Rc::new(RefCell::new(None::<Rc<dyn StdError>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Rc<dyn StdError>>)), false)
        }
    });
    let (mut textCase, _) = error_text(text.clone());
    let (mut textErr, mut textErrOK) = ({
        let val = text.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
                ({ let __err_source = typed_val; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, true)
            } else if let Some(typed_val) = any_val.downcast_ref::<notFound>() {
                (Rc::new(RefCell::new(Some(Rc::new(typed_val.clone()) as Rc<dyn StdError>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<pointerError>>>>() {
                (Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<notFound>>>>() {
                (Rc::new(RefCell::new(Some(Rc::new(GoPointerError::new(typed_val.clone())) as Rc<dyn StdError>))), true)
            } else {
                (Rc::new(RefCell::new(None::<Rc<dyn StdError>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Rc<dyn StdError>>)), false)
        }
    });
    println!("{} {} {} {}", format!("{}", textOK), format!("{}", textCase), format!("{}", textErrOK), format!("{}", { let __nil_result = (*textErr.borrow()).is_none(); __nil_result }));

    let _ = &valueText;
    let _ = &pointerErr;
}