use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};


#[derive(Clone)]
pub struct GoLocalPtrKey<T>(pub Rc<RefCell<Option<T>>>);

impl<T> GoLocalPtrKey<T> {
    pub fn new(value: Rc<RefCell<Option<T>>>) -> Self { GoLocalPtrKey(value) }
    pub fn value(&self) -> Rc<RefCell<Option<T>>> { self.0.clone() }
    fn addr(&self) -> usize { Rc::as_ptr(&self.0) as usize }
}

impl<T> PartialEq for GoLocalPtrKey<T> {
    fn eq(&self, other: &Self) -> bool { self.addr() == other.addr() }
}
impl<T> Eq for GoLocalPtrKey<T> {}
impl<T> PartialOrd for GoLocalPtrKey<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl<T> Ord for GoLocalPtrKey<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.addr().cmp(&other.addr()) }
}
impl<T> std::fmt::Debug for GoLocalPtrKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}
impl<T> std::fmt::Display for GoLocalPtrKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Object;

impl std::fmt::Display for types_Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Object>")
    }
}


impl types_Object {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Package;

impl std::fmt::Display for types_Package {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Package>")
    }
}


impl types_Package {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some::<String>(Default::default())))
    }
}


pub mod sort {
    use super::*;
    pub fn slice<T0, T1>(_arg0: T0, _arg1: T1) {
    }
}


#[derive(Debug, Clone, Default)]
pub struct pkgObj {
    pub obj: Rc<RefCell<Option<types_Object>>>,
    pub name: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for pkgObj {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.obj.borrow().as_ref().unwrap()), (*self.name.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct bundle {
    pub localpkg: Rc<RefCell<Option<types_Package>>>,
}

impl std::fmt::Display for bundle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.localpkg.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct writer {
    pub p: Rc<RefCell<Option<bundle>>>,
}

impl std::fmt::Display for writer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.p.borrow().as_ref().unwrap()))
    }
}


impl writer {
    pub fn export_path(&self, pkg: Rc<RefCell<Option<types_Package>>>) -> Rc<RefCell<Option<String>>> {
        if { let __left = pkg.clone(); let __right = (*self.p.borrow().as_ref().unwrap()).localpkg.clone(); let __both_nil = (*__left.borrow()).is_none() && (*__right.borrow()).is_none(); let __eq = __both_nil || Rc::ptr_eq(&__left, &__right); __eq } {
        return Rc::new(RefCell::new(Some("".to_string())));
    }
        return (*pkg.borrow_mut().as_mut().unwrap()).name();
    }
}

pub fn export_path(pkg: Rc<RefCell<Option<types_Package>>>) -> Rc<RefCell<Option<String>>> {

    return (*pkg.borrow_mut().as_mut().unwrap()).name();
}

pub fn remember(m: Rc<RefCell<Option<BTreeMap<GoLocalPtrKey<types_Package>, Rc<RefCell<Option<Vec<pkgObj>>>>>>>>, pkg: Rc<RefCell<Option<types_Package>>>, obj: Rc<RefCell<Option<types_Object>>>) {
    { let __map_key = GoLocalPtrKey::new(pkg.clone()); let __map_value = Rc::new(RefCell::new(None)); (*m.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __map_key = GoLocalPtrKey::new(pkg.clone()); let __map_value = { let __slice = { let __map_holder = m.clone(); let __map_guard = __map_holder.borrow(); __map_guard.as_ref().unwrap().get(&GoLocalPtrKey::new(pkg.clone())).cloned().unwrap_or_else(|| Rc::new(RefCell::new(None))) }; (*__slice.borrow_mut()).get_or_insert_with(Vec::new).push(pkgObj { obj: obj.clone(), name: Rc::new(RefCell::new(Some("name".to_string()))), ..Default::default() }); __slice.clone() }; (*m.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    let mut w = Rc::new(RefCell::new(Some(writer { p: Rc::new(RefCell::new(Some(bundle { localpkg: pkg.clone(), ..Default::default() }))).clone(), ..Default::default() })));
    let mut pkgs: Rc<RefCell<Option<Vec<Rc<RefCell<Option<types_Package>>>>>>> = Rc::new(RefCell::new(None));
    for (__range_key, objs) in { let __range_holder = m.clone(); let __range_guard = __range_holder.borrow(); let __range_map = (*__range_guard.as_ref().unwrap()).clone(); drop(__range_guard); __range_map } {
        let p = __range_key.value();
        { let new_val = { let __append_target = pkgs.clone(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).push(p.clone()); __append_target.clone() }; pkgs = new_val; };
        let objs_closure_clone = objs.clone(); sort::slice(objs_closure_clone.clone(), Rc::new(RefCell::new(Some(Box::new(move |i: Rc<RefCell<Option<i32>>>, j: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<bool>>> {
        return Rc::new(RefCell::new(Some((*(*objs_closure_clone.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone().name.borrow().as_ref().unwrap()) < (*(*objs_closure_clone.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone().name.borrow().as_ref().unwrap()))));
    }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<bool>>>>))));
        let _ = (*objs.borrow().as_ref().unwrap()).len();
    }
    let pkgs_closure_clone = pkgs.clone(); sort::slice(pkgs_closure_clone.clone(), Rc::new(RefCell::new(Some(Box::new(move |i: Rc<RefCell<Option<i32>>>, j: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<bool>>> {
        return Rc::new(RefCell::new(Some((*export_path((*pkgs_closure_clone.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone()).borrow().as_ref().unwrap()).clone() < (*export_path((*pkgs_closure_clone.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone()).borrow().as_ref().unwrap()).clone())));
    }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<bool>>>>))));
    let pkgs_closure_clone = pkgs.clone(); let w_closure_clone = w.clone(); sort::slice(pkgs_closure_clone.clone(), Rc::new(RefCell::new(Some(Box::new(move |i: Rc<RefCell<Option<i32>>>, j: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<bool>>> {
        return Rc::new(RefCell::new(Some((*(*w_closure_clone.borrow_mut().as_mut().unwrap()).export_path((*pkgs_closure_clone.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone()).borrow().as_ref().unwrap()).clone() < (*(*w_closure_clone.borrow_mut().as_mut().unwrap()).export_path((*pkgs_closure_clone.borrow().as_ref().unwrap())[((*j.borrow().as_ref().unwrap())) as usize].clone()).borrow().as_ref().unwrap()).clone())));
    }) as Box<dyn FnMut(Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<bool>>>>))));
    { let __range_holder = pkgs.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for p in __range_values.iter() {
        let _ = export_path((*p).clone());
        let _ = (*w.borrow_mut().as_mut().unwrap()).export_path(p.clone());
        let _ = (*p.borrow_mut().as_mut().unwrap()).name();
        let _ = (*m.borrow().as_ref().unwrap()).get(&GoLocalPtrKey::new(p.clone())).map(|__v| __v.borrow().as_ref().unwrap().clone()).unwrap_or_else(|| vec![]);
    } }
    let _ = (*pkgs.borrow().as_ref().unwrap());
}

fn main() {
    if false {
        remember(Rc::new(RefCell::new(None)), Rc::new(RefCell::new(None)), Rc::new(RefCell::new(None)));
    }
    println!("{}", "ok".to_string());
}