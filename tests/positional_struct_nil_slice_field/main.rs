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

#[derive(Debug, Clone)]
pub struct Ident {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Ident {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Ident {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct File {
    pub name: Rc<RefCell<Option<Ident>>>,
    pub imports: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Ident>>>>>>>,
    pub unresolved: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Ident>>>>>>>,
    pub comments: Rc<RefCell<Option<Vec<Rc<RefCell<Option<Ident>>>>>>>,
}

impl File {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: self.name.clone(), imports: self.imports.clone(), unresolved: self.unresolved.clone(), comments: self.comments.clone() }
    }
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.name.borrow().as_ref().unwrap()), format_slice_wrapped(&self.imports), format_slice_wrapped(&self.unresolved), format_slice_wrapped(&self.comments))
    }
}


/// Mirrors go/ast/filter.go:494's positional struct literal where
/// nil is passed for a slice-of-pointer field.
fn main() {
    let mut imports = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Ident { name: Rc::new(RefCell::new(Some("fmt".to_string()))), ..Default::default() }))), Rc::new(RefCell::new(Some(Ident { name: Rc::new(RefCell::new(Some("os".to_string()))), ..Default::default() })))])));
    let mut comments = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Ident { name: Rc::new(RefCell::new(Some("//c".to_string()))), ..Default::default() })))])));
    let mut f = Rc::new(RefCell::new(Some(File { name: Rc::new(RefCell::new(Some(Ident { name: Rc::new(RefCell::new(Some("main".to_string()))), ..Default::default() }))).clone(), imports: imports.clone(), unresolved: Rc::new(RefCell::new(None)), comments: comments.clone(), ..Default::default() })));
    println!("{} {} {} {}", format!("{}", (*(*(*f.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()), format!("{}", (*(*f.borrow().as_ref().unwrap()).imports.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", { let __nil_target = (*f.borrow().as_ref().unwrap()).unresolved.clone(); let __nil_result = (*__nil_target.borrow()).is_none(); __nil_result }), format!("{}", (*(*f.borrow().as_ref().unwrap()).comments.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
}