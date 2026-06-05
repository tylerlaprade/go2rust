use go2rust_stdlib_stubs::*;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

fn format_slice<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
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

fn format_slice_wrapped<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[Arc<Mutex<Option<T>>>]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.lock().unwrap();
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


fn format_slice_wrapped_stringer<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[Arc<Mutex<Option<T>>>]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        format_slice_wrapped_stringer_values(s.as_ref())
    } else {
        "[]".to_string()
    }
}

fn format_slice_wrapped_stringer_values<T>(slice: &[Arc<Mutex<Option<T>>>]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| {
        let inner = v.lock().unwrap();
        match inner.as_ref() {
            Some(value) => value.to_string(),
            None => "<nil>".to_string(),
        }
    }).collect();
    format!("[{}]", formatted.join(" "))
}

#[derive(Clone, Default)]
pub struct ExprList(pub Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>);

impl Display for ExprList {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice_wrapped_stringer(&self.0))
    }
}


pub fn pad_exprs(elts: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>, length: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>> {
    let mut values = Arc::new(Mutex::new(Some({ let __v = (*elts.lock().unwrap().as_ref().unwrap()).clone(); __v })));
    while { let __tmp_x = ((*values.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ({ let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x < __tmp_y } {
        { let new_val = { let __append_target = values.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(Arc::new(Mutex::new(None))); __append_target.clone() }; values = new_val; };
    }
    return values.clone();
}

pub fn pad_named_exprs(elts: Arc<Mutex<Option<ExprList>>>) -> Arc<Mutex<Option<ExprList>>> {
    { let __base = { let __named_slice = (*elts.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __base_guard = __base.lock().unwrap(); let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); drop(__base_guard); __values.push(Arc::new(Mutex::new(None))); Arc::new(Mutex::new(Some(ExprList(Arc::new(Mutex::new(Some(__values))))))) }
}

pub fn nil_literal() -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>> {
    Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None))])))
}

fn main() {
    go_ast::__go_init_all();
    go_token::__go_init_all();
    strings::__go_init_all();

    if false {
        let mut done = GoChannel::<bool>::new_buffered(1 as usize);
        let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send((*pad_exprs(nil_literal(), Arc::new(Mutex::new(Some(2)))).lock().unwrap()).is_some() && (*pad_named_exprs(Arc::new(Mutex::new(None))).lock().unwrap()).is_some());;;
    });
        println!("{}", format!("{}", done.recv().unwrap_or_default()));
    }
    println!("{}", format!("{}", "ok".to_string()));
}