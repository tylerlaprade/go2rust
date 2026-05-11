use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

fn format_slice<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_slice_wrapped<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Rc<RefCell<Option<T>>>]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.borrow();
            match inner.as_ref() {
                Some(value) => format!("&{}", value),
                None => "<nil>".to_string(),
            }
        }).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Command {
    pub args: Rc<RefCell<Option<Vec<String>>>>,
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice(&self.args))
    }
}


pub fn quote(s: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {

    return Rc::new(RefCell::new(Some(format!("{}{}", format!("{}{}", "<".to_string(), (*s.borrow().as_ref().unwrap())), ">".to_string()))));
}

pub fn debug(cmd: Rc<RefCell<Option<Command>>>) -> Rc<RefCell<Option<String>>> {

    let mut args: Rc<RefCell<Option<Vec<String>>>> = Rc::new(RefCell::new(None));
    { let __range_holder = (*cmd.borrow().as_ref().unwrap()).args.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for arg in __range_values.iter() {
        let mut quoted = quote(Rc::new(RefCell::new(Some(arg.clone()))));
        if (*Rc::new(RefCell::new(Some({ let __s = (*quoted.borrow().as_ref().unwrap()).clone(); __s[(1) as usize..(((*quoted.borrow().as_ref().unwrap()).len() as i32) - (1 as i32)) as usize].to_string() }))).borrow().as_ref().unwrap()).clone() != (*arg).clone() || (*Rc::new(RefCell::new(Some({ let __s = (*arg).clone(); let __arg = " ".to_string(); __s.contains(&__arg) }))).borrow().as_ref().unwrap()) {
        { let new_val = { let __append_target = args.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push((*quoted.borrow().as_ref().unwrap()).clone()); __append_target.clone() }; args = new_val; };
    } else {
        { let new_val = { let __append_target = args.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push((*arg).clone()); __append_target.clone() }; args = new_val; };
    }
    } }
    return Rc::new(RefCell::new(Some({ let __parts = (*args.borrow().as_ref().unwrap()).clone(); let __sep = " ".to_string(); __parts.join(&__sep) })));
}

fn main() {
    println!("{}", (*debug(Rc::new(RefCell::new(Some(Command { args: Rc::new(RefCell::new(Some(vec!["go".to_string(), "list ./...".to_string()]))), ..Default::default() })))).borrow().as_ref().unwrap()));
}