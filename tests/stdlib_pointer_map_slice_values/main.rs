#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};


/// Runs an append, __store, for elements whose Go copy is not their Clone,
/// such as a struct of cells. When the append moves the slice to a new
/// backing array, the elements it carried over are replaced with __copy of
/// them, so the old array and the new one share nothing.
trait GoSliceAppendCopying<T> {
    fn go_append_copying(&mut self, __store: impl FnOnce(&mut Self), __copy: impl Fn(&T) -> T);
}

impl<T: Clone> GoSliceAppendCopying<T> for GoSlice<T> {
    fn go_append_copying(&mut self, __store: impl FnOnce(&mut Self), __copy: impl Fn(&T) -> T) {
        let __before = self.clone();
        let __len = self.len();
        __store(self);
        if !self.shares_backing_with(&__before) {
            for __i in 0..__len.min(self.len()) {
                let __value = __copy(&self.as_slice()[__i]);
                self.as_mut_slice()[__i] = __value;
            }
        }
    }
}

#[derive(Clone)]
pub struct pkgObj {
    pub obj: Arc<Mutex<Option<types_Object>>>,
    pub name: Arc<Mutex<Option<String>>>,
}

impl pkgObj {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.obj.clone();
        let __go_clone_1_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            obj: __go_clone_0_0,
            name: __go_clone_1_0,
        }
    }
}


impl Default for pkgObj {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            obj: __go_default_0_0,
            name: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for pkgObj {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.obj.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Clone, Default)]
pub struct bundle {
    pub localpkg: Arc<Mutex<Option<types_Package>>>,
}

impl bundle {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.localpkg.clone();
        Self {
            localpkg: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for bundle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.localpkg.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Clone, Default)]
pub struct writer {
    pub p: Arc<Mutex<Option<bundle>>>,
}

impl writer {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.p.clone();
        Self {
            p: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for writer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.p.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl writer {
    pub fn export_path(&self, pkg: Arc<Mutex<Option<types_Package>>>) -> Arc<Mutex<Option<String>>> {
        if {
            let __left = pkg.clone();
            let __right = (*self.p.lock().unwrap().as_ref().unwrap()).localpkg.clone();
            let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
            let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
            __eq
        } {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
        { let __recv = pkg.clone(); let __recv_ptr: *mut types_Package = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut types_Package }; let __result = unsafe { &mut *__recv_ptr }.name(); __result }
    }
}

pub fn export_path(pkg: Arc<Mutex<Option<types_Package>>>) -> Arc<Mutex<Option<String>>> {
    { let __recv = pkg.clone(); let __recv_ptr: *mut types_Package = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut types_Package }; let __result = unsafe { &mut *__recv_ptr }.name(); __result }
}

pub fn remember(m: Arc<Mutex<Option<BTreeMap<GoPtrKey<types_Package>, Arc<Mutex<Option<Vec<pkgObj>>>>>>>>, pkg: Arc<Mutex<Option<types_Package>>>, obj: Arc<Mutex<Option<types_Object>>>) {
    { let __map_key = GoPtrKey::new(pkg.clone()); let __map_value = Arc::new(Mutex::new(None)); (*m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __map_key = GoPtrKey::new(pkg.clone()); let __map_value = { let __slice = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().unwrap().get(&GoPtrKey::new(pkg.clone())).cloned().unwrap_or_else(|| Arc::new(Mutex::new(None))) }; let __append_value = pkgObj { obj: obj.clone(), name: Arc::new(Mutex::new(Some("name".to_string()))), ..Default::default() }; let mut __appended = (*__slice.lock().unwrap()).clone().unwrap_or_default(); __appended.go_append_copying(|__go_slice| { __go_slice.push(__append_value); }, |__go_elem| __go_elem.__go_value_clone()); Arc::new(Mutex::new(Some(__appended))) }; (*m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    let mut w = Arc::new(Mutex::new(Some(writer {
        p: Arc::new(Mutex::new(Some(bundle { localpkg: pkg.clone(), ..Default::default() }))).clone(),
        ..Default::default()
    })));
    let mut pkgs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<types_Package>>>>>>> = Arc::new(Mutex::new(None));
    for (__range_key, objs) in { let __range_holder = m.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        let p = __range_key.value();
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = pkgs.clone(); let __append_value = p.clone(); let mut __appended: Vec<Arc<Mutex<Option<types_Package>>>> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *pkgs.lock().unwrap() = __slice_header; };
        let objs_closure_clone = objs.clone(); { let __sort_target = objs_closure_clone.clone(); let __sort_less = Arc::new(Mutex::new(Some(Box::new(move |i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>| -> bool {
        return {
            let __tmp_x = { let __selector_holder = { let __seq = { let __seq_holder = objs_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned };
            let __tmp_y = { let __selector_holder = { let __seq = { let __seq_holder = objs_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned };
            __tmp_x < __tmp_y
        };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> bool + Send + Sync>))); let __sort_len = { let __sort_guard = __sort_target.lock().unwrap(); __sort_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; for __sort_i in 1..__sort_len { let mut __sort_j = __sort_i; while __sort_j > 0 { let __should_swap = { let mut __less_guard = __sort_less.lock().unwrap(); let __less = __less_guard.as_mut().expect("sort.Slice less function is nil"); __less(Arc::new(Mutex::new(Some(__sort_j as i64))), Arc::new(Mutex::new(Some((__sort_j - 1) as i64)))) }; if !__should_swap { break; } { let mut __sort_guard = __sort_target.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.swap(__sort_j, __sort_j - 1); } } __sort_j -= 1; } } };
        let _ = ({ let __len = (*objs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len });
    }
    let pkgs_closure_clone = pkgs.clone(); { let __sort_target = pkgs_closure_clone.clone(); let __sort_less = Arc::new(Mutex::new(Some(Box::new(move |i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>| -> bool {
        return {
            let __tmp_x = (*export_path({ let __seq = { let __seq_holder = pkgs_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }).lock().unwrap().as_ref().unwrap()).clone();
            let __tmp_y = (*export_path({ let __seq = { let __seq_holder = pkgs_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }).lock().unwrap().as_ref().unwrap()).clone();
            __tmp_x < __tmp_y
        };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> bool + Send + Sync>))); let __sort_len = { let __sort_guard = __sort_target.lock().unwrap(); __sort_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; for __sort_i in 1..__sort_len { let mut __sort_j = __sort_i; while __sort_j > 0 { let __should_swap = { let mut __less_guard = __sort_less.lock().unwrap(); let __less = __less_guard.as_mut().expect("sort.Slice less function is nil"); __less(Arc::new(Mutex::new(Some(__sort_j as i64))), Arc::new(Mutex::new(Some((__sort_j - 1) as i64)))) }; if !__should_swap { break; } { let mut __sort_guard = __sort_target.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.swap(__sort_j, __sort_j - 1); } } __sort_j -= 1; } } };
    let pkgs_closure_clone = pkgs.clone(); let w_closure_clone = w.clone(); { let __sort_target = pkgs_closure_clone.clone(); let __sort_less = Arc::new(Mutex::new(Some(Box::new(move |i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>| -> bool {
        return {
            let __tmp_x = (*{ let __recv = w_closure_clone.clone(); let __recv_ptr: *const writer = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const writer }; let __result = unsafe { &*__recv_ptr }.export_path({ let __seq = { let __seq_holder = pkgs_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }); __result }.lock().unwrap().as_ref().unwrap()).clone();
            let __tmp_y = (*{ let __recv = w_closure_clone.clone(); let __recv_ptr: *const writer = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const writer }; let __result = unsafe { &*__recv_ptr }.export_path({ let __seq = { let __seq_holder = pkgs_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }); __result }.lock().unwrap().as_ref().unwrap()).clone();
            __tmp_x < __tmp_y
        };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> bool + Send + Sync>))); let __sort_len = { let __sort_guard = __sort_target.lock().unwrap(); __sort_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; for __sort_i in 1..__sort_len { let mut __sort_j = __sort_i; while __sort_j > 0 { let __should_swap = { let mut __less_guard = __sort_less.lock().unwrap(); let __less = __less_guard.as_mut().expect("sort.Slice less function is nil"); __less(Arc::new(Mutex::new(Some(__sort_j as i64))), Arc::new(Mutex::new(Some((__sort_j - 1) as i64)))) }; if !__should_swap { break; } { let mut __sort_guard = __sort_target.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.swap(__sort_j, __sort_j - 1); } } __sort_j -= 1; } } };
    { let __range_holder = pkgs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for p in __range_values.iter() {
        let _ = export_path(p.clone());
        let _ = { let __recv = w.clone(); let __recv_ptr: *const writer = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const writer }; let __result = unsafe { &*__recv_ptr }.export_path(p.clone()); __result };
        let _ = { let __recv = p.clone(); let __recv_ptr: *mut types_Package = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut types_Package }; let __result = unsafe { &mut *__recv_ptr }.name(); __result };
        let _ = { let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoPtrKey::new(p.clone()))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };
    } }
    let _ = &pkgs;
}

fn main() {
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();

    if false {
        remember(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));
    }
    println!("{}", format!("{}", "ok".to_string()));
}

impl GoValueClone for pkgObj {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for bundle {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for writer {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
