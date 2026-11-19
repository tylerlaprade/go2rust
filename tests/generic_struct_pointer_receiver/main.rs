use std::any::Any;
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
pub struct P {
    pub x: Rc<RefCell<Option<i32>>>,
}

impl P {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
        }
    }
}


impl Default for P {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            x: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for P {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub struct C<T: Any + 'static> {
    pub p: Rc<RefCell<Option<P>>>,
    pub items: Rc<RefCell<Option<[P; 2]>>>,
    pub v: Rc<RefCell<Option<T>>>,
    pub n: Rc<RefCell<Option<i32>>>,
}

impl<T: Any + 'static> C<T> {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.p.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_1_0 = { let __guard = self.items.borrow(); Rc::new(RefCell::new((*__guard).as_ref().map(|__go_field| __go_field.each_ref().map(|__go_elem| __go_elem.__go_value_clone())))) };
        let __go_clone_2_0 = self.v.clone();
        let __go_clone_3_0 = { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            p: __go_clone_0_0,
            items: __go_clone_1_0,
            v: __go_clone_2_0,
            n: __go_clone_3_0,
        }
    }
}

impl<T: Any + 'static> Clone for C<T> {
    fn clone(&self) -> Self {
        Self {
            p: self.p.clone(),
            items: self.items.clone(),
            v: self.v.clone(),
            n: self.n.clone(),
        }
    }
}


impl<T: Any + 'static> Default for C<T> {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(P::default())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(std::array::from_fn(|_| Default::default()))));
        let __go_default_2_0 = Rc::new(RefCell::new(None));
        let __go_default_3_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            p: __go_default_0_0,
            items: __go_default_1_0,
            v: __go_default_2_0,
            n: __go_default_3_0,
        }
    }
}

impl<T: Any + 'static> std::fmt::Display for C<T> where T: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.p.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.items));
        let __go_fmt_2 = format!("{}", (*self.v.borrow().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.n.borrow().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}


impl<T: Any + 'static> C<T> {
    pub fn later_nested(&mut self) -> Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>> {
        let mut c_closure_clone = (*self).clone(); Rc::new(RefCell::new(Some(Box::new(move || {
        { let __target = (*c_closure_clone.p.borrow().as_ref().unwrap()).x.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }) as Box<dyn FnMut() -> ()>)))
    }

    pub fn later_direct(&mut self) -> Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>> {
        let mut c_closure_clone = (*self).clone(); Rc::new(RefCell::new(Some(Box::new(move || {
        { let __target = c_closure_clone.n.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }) as Box<dyn FnMut() -> ()>)))
    }

    pub fn later_items(&mut self) -> Rc<RefCell<Option<Box<dyn FnMut() -> ()>>>> {
        let mut c_closure_clone = (*self).clone(); Rc::new(RefCell::new(Some(Box::new(move || {
        { let __target = (*c_closure_clone.items.borrow().as_ref().unwrap())[(1) as usize].clone().x.clone(); let __rhs = 5; let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }) as Box<dyn FnMut() -> ()>)))
    }

    pub fn self_(&self) -> Rc<RefCell<Option<C<T>>>> {
        Rc::new(RefCell::new(Some(self.clone())))
    }
}

/// A pointer method of a struct with type parameters can hand on its
/// receiver: a closure over it and a returned receiver are the same struct.
fn main() {
    let mut c = Rc::new(RefCell::new(Some(C::<i32> { ..Default::default() })));
    let mut f = (*c.borrow_mut().as_mut().unwrap()).later_nested();
    { let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = f.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    { let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = f.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    println!("{} {}", format!("{}", "nested field through a closure".to_string()), format!("{}", (*(*(*c.borrow().as_ref().unwrap()).p.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));

    let mut g = (*c.borrow_mut().as_mut().unwrap()).later_direct();
    { let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = g.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    println!("{} {}", format!("{}", "direct field through a closure".to_string()), format!("{}", (*(*c.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())));

    let mut k = (*c.borrow_mut().as_mut().unwrap()).later_items();
    { let __f_ptr: *mut Box<dyn FnMut() -> ()> = { let mut __f_guard = k.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    println!("{} {}", format!("{}", "array of structs through a closure".to_string()), format!("{}", (*(*(*c.borrow().as_ref().unwrap()).items.borrow().as_ref().unwrap())[(1) as usize].clone().x.borrow().as_ref().unwrap())));

    let mut d = (*c.borrow().as_ref().unwrap()).self_();
    { let new_val = 50; *(*d.borrow().as_ref().unwrap()).n.borrow_mut() = Some(new_val); };
    { let new_val = 60; *(*(*d.borrow().as_ref().unwrap()).p.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    println!("{} {} {}", format!("{}", "returned receiver".to_string()), format!("{}", (*(*c.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())), format!("{}", (*(*(*c.borrow().as_ref().unwrap()).p.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())));

    let mut v = Rc::new(RefCell::new(Some({ let __v = (*c.borrow().as_ref().unwrap()).clone(); __v }.__go_value_clone())));
    { let new_val = 7; *(*v.borrow().as_ref().unwrap()).n.borrow_mut() = Some(new_val); };
    { let new_val = 8; *(*(*v.borrow().as_ref().unwrap()).p.borrow().as_ref().unwrap()).x.borrow_mut() = Some(new_val); };
    { let new_val = 9; *(*(*v.borrow().as_ref().unwrap()).items.borrow().as_ref().unwrap())[(1) as usize].clone().x.borrow_mut() = Some(new_val); };
    println!("{} {} {} {} {} {} {}", format!("{}", "copy".to_string()), format!("{}", (*(*c.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())), format!("{}", (*(*(*c.borrow().as_ref().unwrap()).p.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())), format!("{}", (*(*(*c.borrow().as_ref().unwrap()).items.borrow().as_ref().unwrap())[(1) as usize].clone().x.borrow().as_ref().unwrap())), format!("{}", (*(*v.borrow().as_ref().unwrap()).n.borrow().as_ref().unwrap())), format!("{}", (*(*(*v.borrow().as_ref().unwrap()).p.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap())), format!("{}", (*(*(*v.borrow().as_ref().unwrap()).items.borrow().as_ref().unwrap())[(1) as usize].clone().x.borrow().as_ref().unwrap())));
}