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
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for Ident {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
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
        let __go_clone_0_0 = self.name.clone();
        let __go_clone_1_0 = self.imports.clone();
        let __go_clone_2_0 = self.unresolved.clone();
        let __go_clone_3_0 = self.comments.clone();
        Self {
            name: __go_clone_0_0,
            imports: __go_clone_1_0,
            unresolved: __go_clone_2_0,
            comments: __go_clone_3_0,
        }
    }
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.name.borrow(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", format_slice_wrapped(&self.imports));
        let __go_fmt_2 = format!("{}", format_slice_wrapped(&self.unresolved));
        let __go_fmt_3 = format!("{}", format_slice_wrapped(&self.comments));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}


/// Mirrors go/ast/filter.go:494's positional struct literal where
/// nil is passed for a slice-of-pointer field.
fn main() {
    let mut imports = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Ident { name: Rc::new(RefCell::new(Some("fmt".to_string()))), ..Default::default() }))), Rc::new(RefCell::new(Some(Ident { name: Rc::new(RefCell::new(Some("os".to_string()))), ..Default::default() })))])));
    let mut comments = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Ident { name: Rc::new(RefCell::new(Some("//c".to_string()))), ..Default::default() })))])));
    let mut f = Rc::new(RefCell::new(Some(File { name: Rc::new(RefCell::new(Some(Ident { name: Rc::new(RefCell::new(Some("main".to_string()))), ..Default::default() }))).clone(), imports: imports.clone(), unresolved: Rc::new(RefCell::new(None)), comments: comments.clone(), ..Default::default() })));
    println!("{} {} {} {}", format!("{}", (*(*(*f.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()), format!("{}", ({ let __len_target = { let __field = (*f.borrow().as_ref().unwrap()).imports.clone(); __field }; let __len_guard = __len_target.borrow(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })), format!("{}", { let __nil_target = (*f.borrow().as_ref().unwrap()).unresolved.clone(); let __nil_result = (*__nil_target.borrow()).is_none(); __nil_result }), format!("{}", ({ let __len_target = { let __field = (*f.borrow().as_ref().unwrap()).comments.clone(); __field }; let __len_guard = __len_target.borrow(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })));
}