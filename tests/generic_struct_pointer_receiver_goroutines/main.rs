#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
include!("__go2rust_helpers.rs");
mod p;
use p::*;

use std::fmt::{Formatter};
use std::thread;

pub struct C<T: Any + Send + Sync + 'static> {
    pub p: Arc<Mutex<Option<P>>>,
    pub items: Arc<Mutex<Option<[P; 2]>>>,
    pub v: Arc<Mutex<Option<T>>>,
    pub n: Arc<Mutex<Option<i64>>>,
}

impl<T: Any + GoTypeParamFieldClone + Send + Sync + 'static> C<T> {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.p.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_1_0 = { let __guard = self.items.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.each_ref().map(|__go_elem| __go_elem.__go_value_clone())))) };
        let __go_clone_2_0 = <T as GoTypeParamFieldClone>::go_type_param_field_clone(&self.v);
        let __go_clone_3_0 = { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            p: __go_clone_0_0,
            items: __go_clone_1_0,
            v: __go_clone_2_0,
            n: __go_clone_3_0,
        }
    }
}

impl<T: Any + Send + Sync + 'static> Clone for C<T> {
    fn clone(&self) -> Self {
        Self {
            p: self.p.clone(),
            items: self.items.clone(),
            v: self.v.clone(),
            n: self.n.clone(),
        }
    }
}


impl<T: Any + Send + Sync + 'static> Default for C<T> {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(P::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(std::array::from_fn::<_, 2, _>(|_| Default::default()))));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            p: __go_default_0_0,
            items: __go_default_1_0,
            v: __go_default_2_0,
            n: __go_default_3_0,
        }
    }
}

impl<T: Any + Send + Sync + 'static> std::fmt::Display for C<T> where T: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.p.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.items));
        let __go_fmt_2 = format!("{}", (*self.v.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.n.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}


pub struct Outer<T: Any + Send + Sync + 'static> {
    pub r#box: Arc<Mutex<Option<C<T>>>>,
}

impl<T: Any + GoTypeParamFieldClone + Send + Sync + 'static> Outer<T> {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.r#box.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            r#box: __go_clone_0_0,
        }
    }
}

impl<T: Any + Send + Sync + 'static> Clone for Outer<T> {
    fn clone(&self) -> Self {
        Self {
            r#box: self.r#box.clone(),
        }
    }
}


impl<T: Any + Send + Sync + 'static> Default for Outer<T> {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        Self {
            r#box: __go_default_0_0,
        }
    }
}

impl<T: Any + Send + Sync + 'static> std::fmt::Display for Outer<T> where T: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.r#box.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl<T: Any + Send + Sync + 'static> C<T> {
    pub fn later_nested(&mut self) -> Arc<Mutex<Option<Arc<dyn Fn() -> () + Send + Sync>>>> {
        let mut c_closure_clone = (*self).clone(); Arc::new(Mutex::new(Some(Arc::new(move || {
        let mut c_closure_clone = c_closure_clone.clone();
        { let __target = (*c_closure_clone.p.lock().unwrap().as_ref().unwrap()).x.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }) as Arc<dyn Fn() -> () + Send + Sync>)))
    }

    pub fn later_direct(&mut self) -> Arc<Mutex<Option<Arc<dyn Fn() -> () + Send + Sync>>>> {
        let mut c_closure_clone = (*self).clone(); Arc::new(Mutex::new(Some(Arc::new(move || {
        let mut c_closure_clone = c_closure_clone.clone();
        { let __target = c_closure_clone.n.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }) as Arc<dyn Fn() -> () + Send + Sync>)))
    }

    pub fn later_items(&mut self) -> Arc<Mutex<Option<Arc<dyn Fn() -> () + Send + Sync>>>> {
        let mut c_closure_clone = (*self).clone(); Arc::new(Mutex::new(Some(Arc::new(move || {
        let mut c_closure_clone = c_closure_clone.clone();
        { let __target = { let __seq = { let __seq_holder = c_closure_clone.items.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.x.clone(); let __rhs = 5; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }) as Arc<dyn Fn() -> () + Send + Sync>)))
    }

    pub fn self_(&self) -> Arc<Mutex<Option<C<T>>>> {
        if let Some(mut c) = go_take_receiver_handle::<Self>(self as *const Self as usize) {
        return c.clone();
        } else {
        Arc::new(Mutex::new(Some(self.clone())))
        }
    }
}

impl<T: Any + GoTypeParamFieldClone + Send + Sync + 'static> C<T> {
    /// A value receiver is copied even when its body does not otherwise mention
    /// the type-parameter field.
    pub fn bump_value(&self) {
        let mut __self = self.__go_value_clone();
        { let __target = __self.n.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

impl<T: Any + GoValueClone + GoTypeParamFieldClone + Send + Sync + 'static> C<T> {
    /// The implicit receiver copy and the direct type-parameter assignment require
    /// both generated copy traits on T.
    pub fn store_value(&self, value: Arc<Mutex<Option<T>>>) {
        let mut __self = self.__go_value_clone();
        { let new_val = (*value.lock().unwrap().as_ref().unwrap()).go_value_clone(); *__self.v.lock().unwrap() = Some(new_val); };
    }
}

pub fn copy_c<T: Any + GoTypeParamFieldClone + Send + Sync + 'static>(value: Arc<Mutex<Option<C<T>>>>) -> Arc<Mutex<Option<C<T>>>> {
    return { let __owned = value.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };
}

pub fn store_and_copy<T: Any + GoValueClone + GoTypeParamFieldClone + Send + Sync + 'static>(c: Arc<Mutex<Option<C<T>>>>, value: Arc<Mutex<Option<T>>>) -> Arc<Mutex<Option<C<T>>>> {
    { let new_val = (*value.lock().unwrap().as_ref().unwrap()).go_value_clone(); *(*c.lock().unwrap().as_ref().unwrap()).v.lock().unwrap() = Some(new_val); };
    return { let __owned = c.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };
}

/// A pointer method of a struct with type parameters can hand on its
/// receiver: a closure over it and a returned receiver are the same struct.
fn main() {
    go_install_panic_exit();
        // A goroutine makes every variable a locked one.
    let mut done = GoChannel::<String>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        done_thread.send("started".to_string());;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));

    let mut c = Arc::new(Mutex::new(Some(C::<i64> { ..Default::default() })));
    let mut f = { let __recv = c.clone(); let __recv_ptr: *mut C<i64> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut C<i64> }; let __result = unsafe { &mut *__recv_ptr }.later_nested(); __result };
    let mut f = Arc::new(Mutex::new(GoFuncValue::go_func_value(&f)));
    {
        let __f: Arc<dyn Fn() -> () + Send + Sync> = f.lock().unwrap().as_ref().unwrap().clone();
        (__f)(

        )
    };
    {
        let __f: Arc<dyn Fn() -> () + Send + Sync> = f.lock().unwrap().as_ref().unwrap().clone();
        (__f)(

        )
    };
    println!("{} {}", format!("{}", "nested field through a closure".to_string()), format!("{}", (*(*(*c.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap())));

    let mut g = { let __recv = c.clone(); let __recv_ptr: *mut C<i64> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut C<i64> }; let __result = unsafe { &mut *__recv_ptr }.later_direct(); __result };
    let mut g = Arc::new(Mutex::new(GoFuncValue::go_func_value(&g)));
    {
        let __f: Arc<dyn Fn() -> () + Send + Sync> = g.lock().unwrap().as_ref().unwrap().clone();
        (__f)(

        )
    };
    println!("{} {}", format!("{}", "direct field through a closure".to_string()), format!("{}", (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap())));

    let mut k = { let __recv = c.clone(); let __recv_ptr: *mut C<i64> = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut C<i64> }; let __result = unsafe { &mut *__recv_ptr }.later_items(); __result };
    let mut k = Arc::new(Mutex::new(GoFuncValue::go_func_value(&k)));
    {
        let __f: Arc<dyn Fn() -> () + Send + Sync> = k.lock().unwrap().as_ref().unwrap().clone();
        (__f)(

        )
    };
    println!("{} {}", format!("{}", "array of structs through a closure".to_string()), format!("{}", (*{ let __seq = { let __seq_holder = (*c.lock().unwrap().as_ref().unwrap()).items.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())));

    let mut d = { let __recv = c.clone(); let __recv_ptr = go_hint_receiver(&__recv); let __result = unsafe { &*__recv_ptr }.self_(); __result };
    { let new_val = 50; *(*d.lock().unwrap().as_ref().unwrap()).n.lock().unwrap() = Some(new_val); };
    { let new_val = 60; *(*(*d.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "returned receiver".to_string()), format!("{}", (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", (*(*(*c.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap())));

    let mut v = Arc::new(Mutex::new(Some({ let __v = (*c.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v })));
    { let new_val = 7; *(*v.lock().unwrap().as_ref().unwrap()).n.lock().unwrap() = Some(new_val); };
    { let new_val = 8; *(*(*v.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    { let new_val = 9; *{ let __seq = { let __seq_holder = (*v.lock().unwrap().as_ref().unwrap()).items.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.x.lock().unwrap() = Some(new_val); };
    println!("{} {} {} {} {} {} {}", format!("{}", "copy".to_string()), format!("{}", (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", (*(*(*c.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __seq = { let __seq_holder = (*c.lock().unwrap().as_ref().unwrap()).items.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*v.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", (*(*(*v.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __seq = { let __seq_holder = (*v.lock().unwrap().as_ref().unwrap()).items.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.x.lock().unwrap().as_ref().unwrap())));

        // The instantiated type parameter is a field value too. Copying the
        // generic struct must copy a concrete struct value rather than sharing its
        // generated field cells.
    let mut originalValuePtr = Arc::new(Mutex::new(Some(C::<P> { v: Arc::new(Mutex::new(Some(P { x: Arc::new(Mutex::new(Some(11))), ..Default::default() }))), ..Default::default() })));
    let mut originalValue = Arc::new(Mutex::new(Some({ let __v = (*originalValuePtr.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v })));
    let mut copiedValue = { let __owned = originalValue.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };
    { let new_val = 12; *(*(*copiedValue.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "type parameter struct copy".to_string()), format!("{}", (*(*(*originalValue.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap())), format!("{}", (*(*(*copiedValue.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap())));

        // A slice is itself a handle in Go: the generic field cell is independent,
        // while the slice's backing storage remains shared by the copied header.
    let mut originalSlicePtr = Arc::new(Mutex::new(Some(C::<Vec<i64>> { v: Arc::new(Mutex::new(Some(vec![13i64]))), ..Default::default() })));
    let mut originalSlice = Arc::new(Mutex::new(Some({ let __v = (*originalSlicePtr.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v })));
    let mut copiedSlice = { let __owned = originalSlice.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };
    { let __idx = (0) as usize; let __rhs = 14; (*(*copiedSlice.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {} {}", format!("{}", "type parameter slice elements".to_string()), format!("{}", { let __seq = { let __seq_holder = (*originalSlice.lock().unwrap().as_ref().unwrap()).v.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }), format!("{}", { let __seq = { let __seq_holder = (*copiedSlice.lock().unwrap().as_ref().unwrap()).v.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }));
    { let new_val = Arc::new(Mutex::new(Some(vec![15i64]))); (*copiedSlice.lock().unwrap().as_mut().unwrap()).v = new_val; };
    println!("{} {} {}", format!("{}", "type parameter slice field".to_string()), format!("{}", { let __seq = { let __seq_holder = (*originalSlice.lock().unwrap().as_ref().unwrap()).v.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }), format!("{}", { let __seq = { let __seq_holder = (*copiedSlice.lock().unwrap().as_ref().unwrap()).v.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }));

        // Arrays remain Go values when they instantiate the type parameter.
    let mut originalArrayPtr = Arc::new(Mutex::new(Some(C::<[i64; 2]> { v: Arc::new(Mutex::new(Some([16i64, 17i64]))), ..Default::default() })));
    let mut originalArray = Arc::new(Mutex::new(Some({ let __v = (*originalArrayPtr.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v })));
    let mut copiedArray = { let __owned = originalArray.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };
    { let __idx = (0) as usize; let __rhs = 18; (*(*copiedArray.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {} {}", format!("{}", "type parameter array copy".to_string()), format!("{}", { let __seq = { let __seq_holder = (*originalArray.lock().unwrap().as_ref().unwrap()).v.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }), format!("{}", { let __seq = { let __seq_holder = (*copiedArray.lock().unwrap().as_ref().unwrap()).v.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }));

        // Named scalar instantiations implement the generated copy traits, and a
        // generic function returning C[T] propagates the stronger bound.
    let mut originalNumberPtr = Arc::new(Mutex::new(Some(C::<Number> { v: Arc::new(Mutex::new(Some(Number(Arc::new(Mutex::new(Some(19 as i64))))))), ..Default::default() })));
    let mut originalNumber = Arc::new(Mutex::new(Some({ let __v = (*originalNumberPtr.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v })));
    let mut copiedNumber = copy_c::<Number>(originalNumber.clone());
    { let new_val = Number(Arc::new(Mutex::new(Some(20 as i64)))); *(*copiedNumber.lock().unwrap().as_ref().unwrap()).v.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "type parameter named scalar".to_string()), format!("{}", (*{ let __field = (*originalNumber.lock().unwrap().as_ref().unwrap()).v.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", (*{ let __field = (*copiedNumber.lock().unwrap().as_ref().unwrap()).v.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()));

        // A nested generic struct propagates the field-copy bound through C[T].
    let mut originalOuterPtr = Arc::new(Mutex::new(Some(Outer::<P> { r#box: Arc::new(Mutex::new(Some(C::<P> { v: Arc::new(Mutex::new(Some(P { x: Arc::new(Mutex::new(Some(21))), ..Default::default() }))), ..Default::default() }))), ..Default::default() })));
    let mut originalOuter = Arc::new(Mutex::new(Some({ let __v = (*originalOuterPtr.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v })));
    let mut copiedOuter = { let __owned = originalOuter.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };
    { let new_val = 22; *(*(*(*copiedOuter.lock().unwrap().as_ref().unwrap()).r#box.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "nested generic struct copy".to_string()), format!("{}", (*(*(*(*originalOuter.lock().unwrap().as_ref().unwrap()).r#box.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap())), format!("{}", (*(*(*(*copiedOuter.lock().unwrap().as_ref().unwrap()).r#box.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap())));

        // An array containing a slice copies the array value but retains the slice
        // backing storage, as Go does.
    let mut shared = Arc::new(Mutex::new(Some(vec![23i64])));
    let mut originalSliceArrayPtr = Arc::new(Mutex::new(Some(C::<[Vec<i64>; 1]> { v: Arc::new(Mutex::new(Some([{ let __slice_holder = shared.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }]))), ..Default::default() })));
    let mut originalSliceArray = Arc::new(Mutex::new(Some({ let __v = (*originalSliceArrayPtr.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v })));
    let mut copiedSliceArray = { let __owned = originalSliceArray.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };
    { let __outer_idx = (0) as usize; let __idx = (0) as usize; let __rhs = 24; (*(*copiedSliceArray.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_mut().unwrap())[__outer_idx][__idx] = __rhs; };
    println!("{} {} {}", format!("{}", "type parameter array slice backing".to_string()), format!("{}", ({ let __seq = { let __seq_holder = (*originalSliceArray.lock().unwrap().as_ref().unwrap()).v.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() })[(0) as usize].clone()), format!("{}", ({ let __seq = { let __seq_holder = (*copiedSliceArray.lock().unwrap().as_ref().unwrap()).v.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() })[(0) as usize].clone()));

    let mut valueReceiverPtr = Arc::new(Mutex::new(Some(C::<i64> { n: Arc::new(Mutex::new(Some(25))), ..Default::default() })));
    let mut valueReceiver = Arc::new(Mutex::new(Some({ let __v = (*valueReceiverPtr.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v })));
    (*valueReceiver.lock().unwrap().as_ref().unwrap()).bump_value();
    println!("{} {}", format!("{}", "generic value receiver".to_string()), format!("{}", (*{ let __field = (*valueReceiver.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap())));

    let mut combinedBoundPtr = Arc::new(Mutex::new(Some(C::<P> { v: Arc::new(Mutex::new(Some(P { x: Arc::new(Mutex::new(Some(26))), ..Default::default() }))), ..Default::default() })));
    let mut combinedBound = Arc::new(Mutex::new(Some({ let __v = (*combinedBoundPtr.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v })));
    (*combinedBound.lock().unwrap().as_ref().unwrap()).store_value(Arc::new(Mutex::new(Some(P { x: Arc::new(Mutex::new(Some(27))), ..Default::default() }))));
    println!("{} {}", format!("{}", "combined generic copy bounds".to_string()), format!("{}", (*(*(*combinedBound.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap())));

    let mut next = Arc::new(Mutex::new(Some(30i64)));
    let mut next_closure_clone = next.clone(); let mut originalFuncPtr = Arc::new(Mutex::new(Some(C::<Arc<dyn Fn() -> i64 + Send + Sync>> {
        v: Arc::new(Mutex::new(Some(Arc::new(move || -> i64 {
        let mut next_closure_clone = next_closure_clone.clone();
        { let mut guard = next_closure_clone.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return { let __v = (*next_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }) as Arc<dyn Fn() -> i64 + Send + Sync>))),
        ..Default::default()
    })));
    let mut originalFunc = Arc::new(Mutex::new(Some({ let __v = (*originalFuncPtr.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v })));
    let mut copiedFunc = { let __owned = originalFunc.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };
    println!("{} {} {}", format!("{}", "function type argument".to_string()), format!("{}", { let __f_holder = (*originalFunc.lock().unwrap().as_ref().unwrap()).v.clone(); let __f: Arc<dyn Fn() -> i64 + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }), format!("{}", { let __f_holder = (*copiedFunc.lock().unwrap().as_ref().unwrap()).v.clone(); let __f: Arc<dyn Fn() -> i64 + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }));
    { let new_val = Arc::new(move || -> i64 {
        99
    }) as Arc<dyn Fn() -> i64 + Send + Sync>; *(*copiedFunc.lock().unwrap().as_ref().unwrap()).v.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "function type argument field".to_string()), format!("{}", { let __f_holder = (*originalFunc.lock().unwrap().as_ref().unwrap()).v.clone(); let __f: Arc<dyn Fn() -> i64 + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }), format!("{}", { let __f_holder = (*copiedFunc.lock().unwrap().as_ref().unwrap()).v.clone(); let __f: Arc<dyn Fn() -> i64 + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }));

    let mut total = Arc::new(Mutex::new(Some(40i64)));
    let mut total_closure_clone = total.clone(); let mut originalUnaryPtr = Arc::new(Mutex::new(Some(C::<Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>> {
        v: Arc::new(Mutex::new(Some(Arc::new(move |value: Arc<Mutex<Option<i64>>>| -> i64 {
        let mut total_closure_clone = total_closure_clone.clone();
        { let __rhs = (*value.lock().unwrap().as_ref().unwrap()); let mut guard = total_closure_clone.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        return { let __v = (*total_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }) as Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>))),
        ..Default::default()
    })));
    let mut originalUnary = Arc::new(Mutex::new(Some({ let __v = (*originalUnaryPtr.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v })));
    let mut copiedUnary = { let __owned = originalUnary.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };
    println!("{} {} {}", format!("{}", "function type argument parameter".to_string()), format!("{}", { let __f_holder = (*originalUnary.lock().unwrap().as_ref().unwrap()).v.clone(); let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)(Arc::new(Mutex::new(Some(1)))) }), format!("{}", { let __f_holder = (*copiedUnary.lock().unwrap().as_ref().unwrap()).v.clone(); let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)(Arc::new(Mutex::new(Some(2)))) }));
    { let new_val = Arc::new(move |value: Arc<Mutex<Option<i64>>>| -> i64 {
        return { let __tmp_x = { let __v = (*value.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x * __tmp_y };
    }) as Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync>; *(*copiedUnary.lock().unwrap().as_ref().unwrap()).v.lock().unwrap() = Some(new_val); };
    println!("{} {} {}", format!("{}", "function type argument parameter field".to_string()), format!("{}", { let __f_holder = (*originalUnary.lock().unwrap().as_ref().unwrap()).v.clone(); let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)(Arc::new(Mutex::new(Some(3)))) }), format!("{}", { let __f_holder = (*copiedUnary.lock().unwrap().as_ref().unwrap()).v.clone(); let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> i64 + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)(Arc::new(Mutex::new(Some(4)))) }));

        // Repeatedly copying the same function field must keep a stable callable
        // target rather than growing a forwarding chain on the source value.
    let mut stableNext = Arc::new(Mutex::new(Some(100i64)));
    let mut stableNext_closure_clone = stableNext.clone(); let mut stableFuncPtr = Arc::new(Mutex::new(Some(C::<Arc<dyn Fn() -> i64 + Send + Sync>> {
        v: Arc::new(Mutex::new(Some(Arc::new(move || -> i64 {
        let mut stableNext_closure_clone = stableNext_closure_clone.clone();
        { let mut guard = stableNext_closure_clone.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return { let __v = (*stableNext_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }) as Arc<dyn Fn() -> i64 + Send + Sync>))),
        ..Default::default()
    })));
    let mut stableFunc = Arc::new(Mutex::new(Some({ let __v = (*stableFuncPtr.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v })));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 100000; __tmp_x < __tmp_y } {
        let mut discarded = { let __owned = stableFunc.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };
        let _ = &discarded;
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{} {}", format!("{}", "function repeated copy".to_string()), format!("{}", { let __f_holder = (*stableFunc.lock().unwrap().as_ref().unwrap()).v.clone(); let __f: Arc<dyn Fn() -> i64 + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }));
    { let new_val = Arc::new(move || -> i64 {
        200
    }) as Arc<dyn Fn() -> i64 + Send + Sync>; *(*stableFunc.lock().unwrap().as_ref().unwrap()).v.lock().unwrap() = Some(new_val); };
    let mut replacementCopy = { let __owned = stableFunc.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };
    println!("{} {} {}", format!("{}", "function repeated copy replacement".to_string()), format!("{}", { let __f_holder = (*stableFunc.lock().unwrap().as_ref().unwrap()).v.clone(); let __f: Arc<dyn Fn() -> i64 + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }), format!("{}", { let __f_holder = (*replacementCopy.lock().unwrap().as_ref().unwrap()).v.clone(); let __f: Arc<dyn Fn() -> i64 + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }));

        // Channel instantiations must still satisfy the generated copy helper.
    let mut originalChannel: Arc<Mutex<Option<C<GoChannel<i64>>>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut copiedChannel = { let __owned = originalChannel.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };
    let _ = &copiedChannel;
}

impl<T: Any + GoTypeParamFieldClone + Send + Sync + 'static> GoValueClone for C<T> {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}

impl<T: Any + GoTypeParamFieldClone + Send + Sync + 'static> GoTypeParamFieldClone for C<T> {
    const GO_COPY_KEEPS_HANDLE: bool = false;
    fn go_type_param_field_clone(field: &Arc<Mutex<Option<Self>>>) -> Arc<Mutex<Option<Self>>> {
        let __guard = field.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.go_value_clone())))
    }
}


impl<T: Any + GoTypeParamFieldClone + Send + Sync + 'static> GoValueClone for Outer<T> {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}

impl<T: Any + GoTypeParamFieldClone + Send + Sync + 'static> GoTypeParamFieldClone for Outer<T> {
    const GO_COPY_KEEPS_HANDLE: bool = false;
    fn go_type_param_field_clone(field: &Arc<Mutex<Option<Self>>>) -> Arc<Mutex<Option<Self>>> {
        let __guard = field.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.go_value_clone())))
    }
}
