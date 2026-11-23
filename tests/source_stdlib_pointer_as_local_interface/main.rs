#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::fmt::{Display};
use std::sync::{Arc, Mutex};

pub trait positioner: std::fmt::Display + Any {
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool;
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>>;
}

impl Clone for Box<dyn positioner + Send + Sync> {
    fn clone(&self) -> Self {
        positioner::__go_clone_box_positioner(self.as_ref())
    }
}

pub trait named: std::fmt::Display + Any {
    fn __go_clone_box_named(&self) -> Box<dyn named + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_named(&self, other: &(dyn named + Send + Sync)) -> bool;
    fn string(&self) -> Arc<Mutex<Option<String>>>;
}

impl Clone for Box<dyn named + Send + Sync> {
    fn clone(&self) -> Self {
        named::__go_clone_box_named(self.as_ref())
    }
}

impl named for go_ast::r#mod::IdentPtr {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::Ident::string(__recv)
        }
    }
    fn __go_clone_box_named(&self) -> Box<dyn named + Send + Sync> {
        Box::new(self.clone()) as Box<dyn named + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_named(&self, other: &(dyn named + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::IdentPtr>() {
            false
        } else {
            false
        }
    }
}

impl positioner for go_ast::r#mod::BasicLitPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::BasicLit::pos(__recv)
        }
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::BasicLitPtr>() {
            false
        } else {
            false
        }
    }
}

impl positioner for go_ast::r#mod::IdentPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::Ident::pos(__recv)
        }
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::IdentPtr>() {
            false
        } else {
            false
        }
    }
}

pub fn at(p: Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>>) -> Arc<Mutex<Option<go_token::position::Pos>>> {
    (*p.lock().unwrap().as_ref().unwrap()).pos()
}

pub fn describe(n: Arc<Mutex<Option<Box<dyn named + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    (*n.lock().unwrap().as_ref().unwrap()).string()
}

fn main() {
    ::go_ast::__go_init_all();
    ::go_token::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut id = go_ast::new_ident(Arc::new(Mutex::new(Some("x".to_string()))));
    println!("{} {}", format!("{}", {
        let __tmp_x = (*at(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(id.clone())) as Box<dyn positioner + Send + Sync>)))).lock().unwrap().as_ref().unwrap()).clone();
        let __tmp_y = (*{ let __recv = id.clone(); let __recv_ptr: *const go_ast::r#mod::Ident = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::Ident }; let __result = unsafe { &*__recv_ptr }.pos(); __result }.lock().unwrap().as_ref().unwrap()).clone();
        __tmp_x == __tmp_y
    }), format!("{}", (*describe(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(id.clone())) as Box<dyn named + Send + Sync>)))).lock().unwrap().as_ref().unwrap())));

    let mut held: Arc<Mutex<Option<Box<dyn named + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(id.clone())) as Box<dyn named + Send + Sync>)));
    { let new_val = "y".to_string(); *(*id.lock().unwrap().as_ref().unwrap()).name.lock().unwrap() = Some(new_val); };
    println!("{} {}", format!("{}", (*(*held.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())), format!("{}", (*describe(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(id.clone())) as Box<dyn named + Send + Sync>)))).lock().unwrap().as_ref().unwrap())));

    let mut lit = Arc::new(Mutex::new(Some(go_ast::r#mod::BasicLit { kind: Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_N_T as i64))))))), value: Arc::new(Mutex::new(Some("1".to_string()))), ..Default::default() })));
    println!("{}", format!("{}", {
        let __tmp_x = (*at(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BasicLitPtr(lit.clone())) as Box<dyn positioner + Send + Sync>)))).lock().unwrap().as_ref().unwrap()).clone();
        let __tmp_y = (*{ let __recv = lit.clone(); let __recv_ptr: *const go_ast::r#mod::BasicLit = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::BasicLit }; let __result = unsafe { &*__recv_ptr }.pos(); __result }.lock().unwrap().as_ref().unwrap()).clone();
        __tmp_x == __tmp_y
    }));
}